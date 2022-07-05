use std::sync::Arc;

use tokio::io::AsyncWriteExt as _;
use tokio::net::TcpStream;
use tracing::debug;

use crate::error::Result;
use crate::session::reply::{Code, Reply};
use crate::{OnNewMail, Session};

pub struct Connection {
    stream: TcpStream,
    session: Session,
    on_new_mail: Arc<OnNewMail>,
}

impl Connection {
    pub fn new(stream: TcpStream, on_new_mail: Arc<OnNewMail>) -> Self {
        Connection {
            stream,
            on_new_mail,
            session: Session::default(),
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let mut reply = Reply::default();

        // open the connection
        reply.code(Code::SERVICE_READY);
        reply.finish();
        self.send(&reply).await?;

        Ok(())
    }

    async fn send(&mut self, reply: &Reply) -> Result<()> {
        debug!("sending {} bytes", reply.data().len());
        self.stream.writable().await?;
        self.stream.write_all(reply.data()).await?;
        Ok(())
    }
}
