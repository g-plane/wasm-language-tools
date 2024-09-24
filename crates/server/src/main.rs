use wat_server::Server;

fn main() -> anyhow::Result<()> {
    let mut server = Server::default();
    server.run()?;
    Ok(())
}
