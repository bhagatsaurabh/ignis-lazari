use activator_core::Bootstrapper;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    tracing::info!("Ignis Lazari starting...");

    let app = Bootstrapper::bootstrap().await;

    app.run().await;
}
