use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncReadExt as _, AsyncWriteExt as _, BufStream};
use tokio::net::TcpStream;
use tracing::debug;

use crate::error::Result;
use crate::session::reply::{Code, Reply};
use crate::{OnNewMail, Session};

pub struct Connection {
    stream: BufStream<TcpStream>,
    session: Session,
    on_new_mail: Arc<OnNewMail>,
    data: Vec<u8>,
}

impl Connection {
    pub fn new(stream: TcpStream, on_new_mail: Arc<OnNewMail>) -> Self {
        Connection {
            stream: BufStream::new(stream),
            on_new_mail,
            session: Session::default(),
            data: Vec::with_capacity(1024),
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let mut reply = Reply::default();

        // open the connection
        reply.code(Code::SERVICE_READY);
        reply.finish();
        self.send(&reply).await?;

        self.line().await?;
        let parsed = crate::session::cmd_parser::CommandParser::new()
            .parse(std::str::from_utf8(&self.data).unwrap())
            .expect("lol");
        debug!(cmd = debug(parsed), "command");

        Ok(())
    }

    async fn send(&mut self, reply: &Reply) -> Result<()> {
        debug!(
            count = display(reply.data().len()),
            bytes = debug(String::from_utf8_lossy(reply.data())),
            "sending",
        );

        self.stream.get_mut().writable().await?;
        self.stream.write_all(reply.data()).await?;
        self.stream.flush().await?;
        Ok(())
    }

    async fn line(&mut self) -> Result<()> {
        self.stream.get_mut().readable().await?;
        self.data.clear();
        loop {
            let count = self.stream.read_until(b'\n', &mut self.data).await?;

            debug!(
                count = display(count),
                bytes = debug(String::from_utf8_lossy(
                    &self.data[(self.data.len() - count)..]
                )),
                "received"
            );

            if count == 0 || self.data.ends_with(b"\r\n") {
                break;
            }
        }
        Ok(())
    }
}
