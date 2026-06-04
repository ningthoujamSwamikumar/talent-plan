pub mod config;
pub mod feature_flags;
pub mod shutdown;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

/// Readiness states for the application.
/// 0 = Starting, 1 = Ready, 2 = Draining, 3 = Stopped.
pub const STATE_STARTING: u8 = 0;
pub const STATE_READY: u8 = 1;
pub const STATE_DRAINING: u8 = 2;
pub const STATE_STOPPED: u8 = 3;

/// Shared application state available to all handlers.
#[derive(Clone)]
pub struct AppState {
    pub readiness: Arc<AtomicU8>,
    pub feature_flags: Arc<feature_flags::FeatureFlagService>,
}

impl AppState {
    pub fn new(flags: feature_flags::FeatureFlagService) -> Self {
        Self {
            readiness: Arc::new(AtomicU8::new(STATE_STARTING)),
            feature_flags: Arc::new(flags),
        }
    }

    /// Mark the application as ready to receive traffic.
    pub fn set_ready(&self) {
        self.readiness.store(STATE_READY, Ordering::SeqCst);
    }

    /// Mark the application as draining (shutdown in progress).
    pub fn set_draining(&self) {
        self.readiness.store(STATE_DRAINING, Ordering::SeqCst);
    }

    /// Mark the application as fully stopped.
    pub fn set_stopped(&self) {
        self.readiness.store(STATE_STOPPED, Ordering::SeqCst);
    }

    /// Return the current readiness state.
    pub fn readiness_state(&self) -> u8 {
        self.readiness.load(Ordering::SeqCst)
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

/// Liveness probe: returns 200 if the process is alive.
async fn healthz() -> impl IntoResponse {
    Json(HealthResponse { status: "alive" })
}

/// Readiness probe: returns 200 only when the service is in the Ready state.
async fn readyz(State(state): State<AppState>) -> impl IntoResponse {
    if state.readiness_state() == STATE_READY {
        (StatusCode::OK, Json(HealthResponse { status: "ready" }))
    } else {
        let label = match state.readiness_state() {
            STATE_STARTING => "starting",
            STATE_DRAINING => "draining",
            STATE_STOPPED => "stopped",
            _ => "unknown",
        };
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse { status: label }),
        )
    }
}

/// A sample gated endpoint that requires the `enable_search` feature flag.
async fn search(State(state): State<AppState>) -> impl IntoResponse {
    if !state.feature_flags.is_enabled("enable_search") {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "search feature is disabled"})),
        );
    }
    (
        StatusCode::OK,
        Json(serde_json::json!({"results": []})),
    )
}

/// Build the application router with all routes and shared state.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/search", get(search))
        .with_state(state)
}
