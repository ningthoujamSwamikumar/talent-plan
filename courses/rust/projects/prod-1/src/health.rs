use std::sync::atomic::Ordering;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use serde_json::json;

use crate::AppState;

/// Liveness probe -- always returns `200 OK`.
///
/// Kubernetes uses this to know the process is running.
pub async fn health_live() -> (StatusCode, Json<serde_json::Value>) {
    todo!("Part 4: return 200 with JSON body {{\"status\": \"alive\"}}")
}

/// Readiness probe -- returns `200 OK` when dependencies are healthy,
/// `503 Service Unavailable` otherwise.
pub async fn health_ready(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    todo!(
        "Part 4: check state.ready flag; return 200 + {{\"status\": \"ready\"}} \
         or 503 + {{\"status\": \"not_ready\", \"reason\": \"...\"}}"
    )
}
