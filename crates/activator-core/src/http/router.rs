use std::sync::Arc;

use axum::Router;
use axum::middleware;
use axum::routing::{get, post};

use super::cors::origin_guard;
use super::handlers::{get_status, start_instance};
use crate::InstanceRegistry;

pub fn build_router(registry: Arc<InstanceRegistry>) -> Router {
    Router::new()
        .route("/v1/instances/:id/status", get(get_status))
        .route("/v1/instances/:id/start", post(start_instance))
        .layer(middleware::from_fn_with_state(
            registry.clone(),
            origin_guard,
        ))
        .with_state(registry)
}
