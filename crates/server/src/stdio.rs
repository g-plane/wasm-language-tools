use crate::message::Message;
use compio::{
    fs::{Stdin, Stdout},
    io::{AsyncRead, AsyncWrite},
};

macro_rules! buf_try {
    ($e:expr) => {{
        let compio::buf::BufResult(result, buf) = $e;
        result?;
        buf
    }};
}

pub struct Stdio {
    stdin: Stdin,
    stdout: Stdout,
}

impl Default for Stdio {
    fn default() -> Self {
        Self {
            stdin: compio::fs::stdin(),
            stdout: compio::fs::stdout(),
        }
    }
}

impl Stdio {
    pub async fn read(&mut self) -> anyhow::Result<Message> {
        let mut s = Vec::new();
        let total_length: usize;
        let buf = buf_try!(self.stdin.read(Vec::with_capacity(30)).await);
        if buf.starts_with(b"Content-Length") {
            if let Some((left, right)) = String::from_utf8_lossy(&buf).split_once("\r\n") {
                total_length = left.trim_start_matches("Content-Length:").trim().parse()?;
                s.reserve_exact(total_length + 2);
                s.extend_from_slice(right.as_bytes());
            } else {
                return Err(anyhow::anyhow!("invalid header `Content-Length`"));
            }
        } else {
            return Err(anyhow::anyhow!("missing header `Content-Length`"));
        }

        let mut buf = buf_try!(
            self.stdin
                .read(Vec::with_capacity(total_length - s.len() + 2))
                .await
        );
        s.append(&mut buf);
        serde_json::from_slice(&s).map_err(anyhow::Error::from)
    }

    pub async fn write(&mut self, message: Message) -> anyhow::Result<()> {
        let mut value = serde_json::to_value(message)?;
        if let serde_json::Value::Object(map) = &mut value {
            map.insert("jsonrpc".into(), "2.0".into());
        }
        let s = serde_json::to_vec(&value)?;
        buf_try!(
            self.stdout
                .write(format!("Content-Length: {}\r\n\r\n", s.len()))
                .await
        );
        buf_try!(self.stdout.write(s).await);
        self.stdout.flush().await?;
        Ok(())
    }
}
