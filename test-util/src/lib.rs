pub fn init() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_writer(tracing_subscriber::fmt::TestWriter::new())
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}
