use crate::message::Message;
use async_lock::Mutex;
use compio::{
    arrayvec::ArrayVec,
    fs::{Stdin, Stdout},
    io::{AsyncRead, AsyncWrite},
};
use std::{io::Read, sync::Arc};
use tracing::{event, Level};

macro_rules! buf_try {
    ($e:expr) => {{
        let compio::buf::BufResult(result, buf) = $e;
        result?;
        buf
    }};
}

#[derive(Clone)]
pub struct Stdio {
    stdin: Stdin,
    stdout: Arc<Mutex<Stdout>>,
}

impl Default for Stdio {
    fn default() -> Self {
        Self {
            stdin: compio::fs::stdin(),
            stdout: Arc::new(Mutex::new(compio::fs::stdout())),
        }
    }
}

impl Stdio {
    pub async fn read(&mut self) -> anyhow::Result<Message> {
        let buf = buf_try!(self.stdin.read(ArrayVec::<_, 30>::new_const()).await);
        let total_length: usize;
        let head;
        if let Some((left, right)) = buf.strip_prefix(b"Content-Length: ").and_then(|rest| {
            rest.iter()
                .position(|c| *c == b'\r')
                .and_then(|i| rest.get(..i).zip(rest.get(i + 4..)))
        }) {
            total_length = String::from_utf8_lossy(left).parse()?;
            head = right;
        } else {
            return Err(anyhow::anyhow!("invalid header `Content-Length`"));
        }

        let tail = buf_try!(
            self.stdin
                .read(Vec::with_capacity(total_length - head.len()))
                .await
        );
        serde_json::from_reader(head.chain(&*tail))
            .inspect(|message| {
                event!(Level::DEBUG, "client → server:\n{message:#?}");
            })
            .map_err(anyhow::Error::from)
    }

    pub async fn write(&self, message: Message) -> anyhow::Result<()> {
        let mut stdout = self.stdout.lock().await;
        let mut value = serde_json::to_value(message)?;
        if let serde_json::Value::Object(map) = &mut value {
            map.insert("jsonrpc".into(), "2.0".into());
        }
        let s = serde_json::to_vec(&value)?;
        buf_try!(
            stdout
                .write(format!("Content-Length: {}\r\n\r\n", s.len()))
                .await
        );
        buf_try!(stdout.write(s).await);
        stdout.flush().await?;
        event!(Level::DEBUG, "server → client:\n{value:#?}");
        Ok(())
    }
}
