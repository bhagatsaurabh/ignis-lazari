use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::InstanceRegistry;

pub async fn origin_guard(
    State(registry): State<Arc<InstanceRegistry>>,
    request: Request,
    next: Next,
) -> Response {
    let Some(id) = extract_instance_id(request.uri().path()) else {
        return next.run(request).await;
    };

    let Ok(instance) = registry.get(&id) else {
        return next.run(request).await; // let the handler produce its own 404
    };

    let origin = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);

    let Some(origin) = origin else {
        return next.run(request).await; // no Origin header: not a browser call, allow
    };

    if !instance
        .allowed_origins
        .iter()
        .any(|allowed| allowed == &origin)
    {
        return (StatusCode::FORBIDDEN, "origin not allowed").into_response();
    }

    let origin_value = HeaderValue::from_str(&origin).unwrap_or(HeaderValue::from_static(""));

    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        let headers = response.headers_mut();
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value);
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("content-type"),
        );
        return response;
    }

    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value);
    response
}

fn extract_instance_id(path: &str) -> Option<String> {
    let mut segments = path.trim_start_matches('/').split('/');
    match (segments.next(), segments.next(), segments.next()) {
        (Some("v1"), Some("instances"), Some(id)) => Some(id.to_string()),
        _ => None,
    }
}
