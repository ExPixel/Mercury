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
