use std::sync::Arc;

use config::ServerConfig;
use tower_http::trace::TraceLayer;

use crate::http::build_router;
use crate::registry::InstanceRegistry;

pub struct Application {
    registry: Arc<InstanceRegistry>,
    server_config: ServerConfig,
}

impl Application {
    pub fn new(registry: InstanceRegistry, server_config: ServerConfig) -> Self {
        Self {
            registry: Arc::new(registry),
            server_config,
        }
    }

    pub fn registry(&self) -> &InstanceRegistry {
        &self.registry
    }

    pub async fn run(self) {
        let addr = format!("{}:{}", self.server_config.host, self.server_config.port);
        let router = build_router(self.registry.clone()).layer(TraceLayer::new_for_http());

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .unwrap_or_else(|err| {
                tracing::error!(%addr, error = %err, "failed to bind");
                std::process::exit(1);
            });

        tracing::info!(%addr, "listening");

        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap_or_else(|err| {
                tracing::error!(error = %err, "server error");
                std::process::exit(1);
            });

        tracing::info!("shutdown complete");
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("received Ctrl+C, shutting down"),
        _ = terminate => tracing::info!("received SIGTERM, shutting down"),
    }
}
