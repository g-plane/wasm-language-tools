use crate::server::Server;
use std::{env, path, time::SystemTime};

mod message;
mod sent;
mod server;
mod stdio;

fn main() -> anyhow::Result<()> {
    let mut log_file = None;
    let mut log_filter = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match &*arg {
            "-v" | "-V" | "--version" => {
                println!("wat_server v{}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--log-dir" => {
                log_file = args.next().map(|dir| {
                    let mut path = path::PathBuf::from(dir);
                    path.push(format!(
                        "wat_server-{}.log",
                        humantime::format_rfc3339_seconds(SystemTime::now()),
                    ));
                    path
                });
            }
            "--log-filter" => {
                log_filter = args.next();
            }
            _ => {}
        }
    }

    if let Some(log_file) = log_file
        && let Ok(log_file) = fern::log_file(log_file)
    {
        let mut dispatch = fern::Dispatch::new().format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {message}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
            ))
        });
        let log_filter = log_filter.unwrap_or_default();
        for filter in log_filter.split(',') {
            match filter.split_once('=') {
                Some((target, level)) => {
                    if let Ok(level) = level.parse() {
                        dispatch = dispatch.level_for(target.to_owned(), level);
                    }
                }
                None => {
                    if let Ok(level) = filter.parse() {
                        dispatch = dispatch.level(level);
                    }
                }
            }
        }
        let _ = dispatch.chain(log_file).apply();
    }

    Server::new().run()
}
