use std::sync::Arc;

use tokio::net::TcpStream;

use crate::error::Result;
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

    pub async fn run(self) -> Result<()> {
        Ok(())
    }
}
