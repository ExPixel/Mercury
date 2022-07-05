use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("error while creating tokio runtime")?;
    rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let server = smtp_server::Server::builder()
        .bind("localhost:8025")
        .on_conn_err(|err| {})
        .on_new_mail(|mail| {})
        .build()
        .context("error while creating server instance")?;
    todo!();
}
