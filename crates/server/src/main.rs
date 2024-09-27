use tracing::{event, Level};
use tracing_subscriber::prelude::*;
use wat_server::Server;

fn main() -> anyhow::Result<()> {
    if let Ok(layer) = tracing_journald::layer() {
        tracing_subscriber::registry().with(layer).init();
    }

    let span = tracing::span!(Level::TRACE, "wat_server");
    let _enter = span.enter();

    event!(Level::INFO, "wat_server starting");
    let mut server = Server::default();
    server.run()?;
    Ok(())
}
