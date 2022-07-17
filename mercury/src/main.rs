/*
    Mercury, mail testing tool
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

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("error while creating tokio runtime")?;
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
    rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let server = smtp_server::Server::builder()
        .bind("localhost:8025")
        .on_conn_err(|err| {})
        .on_new_mail(|mail| {})
        .build()
        .context("error while creating server instance")?;
    server.run().await.map_err(Into::into)
}
