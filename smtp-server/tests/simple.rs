/*
    Mercury SMTP Server
    Copyright (C) 2022 Adolph Celestin

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{error::Error, time::Duration};

use lettre::{SmtpTransport, Transport};
use tokio::task;
use tracing::info;

#[tokio::test]
pub async fn simple_mail_test() -> Result<(), smtp_server::Error> {
    test_util::init();

    let (err_tx, mut err_rx) = tokio::sync::mpsc::channel(1);
    let (mail_tx, mut mail_rx) = tokio::sync::mpsc::channel(1);

    let server = smtp_server::Server::builder()
        .bind("localhost:8025")
        .on_conn_err(move |err| drop(err_tx.try_send(err)))
        .on_new_mail(move |mail| drop(mail_tx.try_send(mail)))
        .build()?;
    let handle = server.handle();

    let err_task = task::spawn(async move { err_rx.recv().await });
    task::spawn(async move {
        mail_rx.recv().await;
        handle.stop();
    });

    let server_task = task::spawn(async move { server.run().await });
    tokio::time::sleep(Duration::from_micros(100)).await; // wait for server to listen

    let (mailer_err_tx, mut mailer_err_rx) =
        tokio::sync::mpsc::channel::<Box<dyn Send + Sync + Error>>(1);
    task::spawn_blocking(move || {
        let email = lettre::Message::builder()
            .from("TestSend <test-send@example.com>".parse().unwrap())
            .to("TestRcpt <test-rcpt@example.com>".parse().unwrap())
            .subject("Test Email")
            .body(String::from("Test email body"))
            .map_err(|err| drop(mailer_err_tx.blocking_send(Box::new(err))))
            .expect("failed to build email");
        let mailer = SmtpTransport::builder_dangerous("localhost")
            .port(8025)
            .build();

        info!("sending email...");
        match mailer.send(&email) {
            Ok(_) => info!("email sent successfully"),
            Err(err) => drop(mailer_err_tx.blocking_send(Box::new(err))),
        }
    });

    let timeout = tokio::time::sleep(Duration::from_secs(5));

    tokio::select! {
        _ = timeout => panic!("timeout"),
        res = server_task => match res {
            Ok(_) => Ok(()),
            Err(err) => panic!("server error: {:?}", err),
        },
        err = mailer_err_rx.recv() => panic!("mailer error: {:?}", err),
        err = err_task => panic!("connection error: {:?}", err)
    }
}
