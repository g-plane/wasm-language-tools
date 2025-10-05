use crate::server::Server;
use std::{env, time::SystemTime};

mod message;
mod sent;
mod server;
mod stdio;

fn main() -> anyhow::Result<()> {
    if env::args().any(|arg| matches!(&*arg, "-v" | "-V" | "--version")) {
        println!("wat_server v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if env::args().any(|arg| arg == "--debug")
        && let Ok(log_file) = fern::log_file(format!(
            "wat_server-{}.log",
            humantime::format_rfc3339_seconds(SystemTime::now()),
        ))
    {
        let _ = fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {message}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    record.level(),
                    record.target(),
                ))
            })
            .level(log::LevelFilter::Debug)
            .chain(log_file)
            .apply();
    }

    Server::new().run()?;
    Ok(())
}
