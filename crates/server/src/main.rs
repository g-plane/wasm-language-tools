use crate::server::Server;
use std::env;
use tracing::{Level, event};
use tracing_subscriber::prelude::*;

mod message;
mod sent;
mod server;
mod stdio;

fn main() -> anyhow::Result<()> {
    if env::args().any(|arg| arg == "-v" || arg == "-V" || arg == "--version") {
        eprintln!("wat_server v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if env::args().any(|arg| arg == "--debug")
        && let Ok(layer) = tracing_journald::layer()
    {
        tracing_subscriber::registry().with(layer).init();
    }

    let span = tracing::span!(Level::TRACE, "wat_server");
    let _enter = span.enter();

    event!(Level::INFO, "wat_server starting");
    Server::default().run()?;
    Ok(())
}
