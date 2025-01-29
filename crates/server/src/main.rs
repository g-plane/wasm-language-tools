use crate::server::Server;
use std::env;
use tracing::{event, Level};
use tracing_subscriber::prelude::*;

mod message;
mod sent;
mod server;
mod stdio;

fn main() -> anyhow::Result<()> {
    if env::args().any(|arg| arg == "-v" || arg == "-V" || arg == "--version") {
        println!("wat_server v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if env::args().any(|arg| arg == "--debug") {
        if let Ok(layer) = tracing_journald::layer() {
            tracing_subscriber::registry().with(layer).init();
        }
    }

    let span = tracing::span!(Level::TRACE, "wat_server");
    let _enter = span.enter();

    event!(Level::INFO, "wat_server starting");
    async_io::block_on(async {
        let mut server = Server::default();
        server.run().await
    })?;
    Ok(())
}
