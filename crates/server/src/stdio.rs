use crate::message::Message;
use anyhow::Result;
use async_task::Task;
use blocking::unblock;
use std::io::{BufRead, Read, Write};
use tracing::{event, Level};

pub fn read() -> Task<Result<Message>> {
    unblock(|| {
        let mut stdin = std::io::stdin().lock();
        let mut buf = String::with_capacity(30);
        stdin.read_line(&mut buf)?;
        let length = buf.trim_start_matches("Content-Length:").trim().parse()?;
        stdin.read_line(&mut buf)?; // empty line
        serde_json::from_reader(stdin.take(length))
            .inspect(|message| {
                event!(Level::DEBUG, "client → server:\n{message:#?}");
            })
            .map_err(anyhow::Error::from)
    })
}

pub fn write(message: Message) -> Task<Result<()>> {
    unblock(|| write_sync(message))
}

pub fn write_sync(message: Message) -> Result<()> {
    let value = serde_json::to_value(message)?;
    let json = serde_json::to_vec(&value)?;
    let mut stdout = std::io::stdout().lock();
    write!(stdout, "Content-Length: {}\r\n\r\n", json.len())?;
    stdout.write_all(&json)?;
    stdout.flush()?;
    event!(Level::DEBUG, "server → client:\n{value:#?}");
    Ok(())
}
