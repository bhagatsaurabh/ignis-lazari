use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::RegistryError;
use plugin_api::ProviderError;

pub enum ApiError {
    NotFound(String),
    Provider(String),
    Internal(String),
}

impl From<RegistryError> for ApiError {
    fn from(err: RegistryError) -> Self {
        match err {
            RegistryError::InstanceNotFound(id) => ApiError::NotFound(id),
            other => ApiError::Internal(other.to_string()),
        }
    }
}

impl From<ProviderError> for ApiError {
    fn from(err: ProviderError) -> Self {
        ApiError::Provider(err.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(id) => (StatusCode::NOT_FOUND, format!("instance '{id}' not found")),
            ApiError::Provider(msg) => (StatusCode::BAD_GATEWAY, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
