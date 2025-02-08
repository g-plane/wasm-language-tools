use crate::message::Message;
use anyhow::Result;
use async_task::Task;
use blocking::unblock;
use std::io::{BufRead, Read, Write};
use tracing::{event, Level};

pub fn read() -> Task<Result<Option<Message>>> {
    unblock(|| {
        let mut length = 0;
        let mut stdin = std::io::stdin().lock();
        let mut buf = String::with_capacity(30);
        // when stdin closed, read size will be 0, so we exit;
        // otherwise caller will be trapped in an infinite loop
        if stdin.read_line(&mut buf)? == 0 {
            return Ok(None);
        }
        while !buf.trim().is_empty() {
            if let Some(value) = buf.strip_prefix("Content-Length:") {
                length = value.trim().parse()?;
            }
            buf.clear();
            stdin.read_line(&mut buf)?;
        }
        serde_json::from_reader(stdin.take(length))
            .inspect(|message| {
                event!(Level::DEBUG, "client → server:\n{message:#?}");
            })
            .map(Some)
            .map_err(anyhow::Error::from)
    })
}

pub fn write(message: Message) -> Task<Result<()>> {
    unblock(|| write_sync(message))
}

pub fn write_sync(message: Message) -> Result<()> {
    event!(Level::DEBUG, "server → client:\n{message:#?}");
    let json = serde_json::to_string(&message)?;
    let mut stdout = std::io::stdout().lock();
    write!(stdout, "Content-Length: {}\r\n\r\n", json.len())?;
    stdout.write_all(json.as_bytes())?;
    stdout.flush()?;
    event!(Level::DEBUG, "stdout: {json}");
    Ok(())
}
