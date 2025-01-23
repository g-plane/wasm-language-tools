mod message;
mod sent;
mod server;
mod stdio;

use crate::server::Server;
use std::{env, sync::Arc};
use tracing::{event, Level};
use tracing_subscriber::prelude::*;

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
    let runtime = Arc::new(compio::runtime::Runtime::new()?);
    runtime.block_on(async {
        let mut server = Server::default();
        server.run(Arc::clone(&runtime)).await
    })?;
    Ok(())
}
