use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use serde::Serialize;

use plugin_api::InstanceState;

use super::error::ApiError;
use crate::InstanceRegistry;

#[derive(Serialize)]
pub struct StatusResponse {
    id: String,
    state: InstanceState,
}

pub async fn get_status(
    State(registry): State<Arc<InstanceRegistry>>,
    Path(id): Path<String>,
) -> Result<Json<StatusResponse>, ApiError> {
    let instance = registry.get(&id)?;
    let state = instance.provider.status().await?;

    Ok(Json(StatusResponse { id, state }))
}

pub async fn start_instance(
    State(registry): State<Arc<InstanceRegistry>>,
    Path(id): Path<String>,
) -> Result<Json<StatusResponse>, ApiError> {
    let instance = registry.get(&id)?;
    instance.provider.start().await?;

    Ok(Json(StatusResponse {
        id,
        state: InstanceState::Starting,
    }))
}
