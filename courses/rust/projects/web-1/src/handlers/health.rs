//! Health-check handler.

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

/// Returns `200 OK` with `{"status": "ok"}`.
///
/// Used by load balancers and integration tests to verify the server is running.
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}
