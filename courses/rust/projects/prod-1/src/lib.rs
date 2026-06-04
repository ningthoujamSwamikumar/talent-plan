#![allow(unused_imports, unused_variables)]

pub mod health;
pub mod metrics_setup;
pub mod middleware;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use axum::extract::State;
use axum::http::Request;
use axum::routing::get;
use axum::Router;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

/// Shared application state available to all handlers.
#[derive(Clone)]
pub struct AppState {
    /// When `true`, the readiness probe reports healthy.
    pub ready: Arc<AtomicBool>,
    /// Handle used to render Prometheus metrics.
    pub metrics_handle: metrics_exporter_prometheus::PrometheusHandle,
}

/// A [`MakeRequestId`] implementation that generates UUID v4 values.
#[derive(Clone, Default)]
pub struct UuidRequestId;

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string();
        Some(RequestId::new(id.parse().unwrap()))
    }
}

/// Initialise the `tracing` subscriber with JSON output and env-filter.
///
/// The filter reads from `RUST_LOG`, defaulting to `info`.
pub fn init_tracing() {
    todo!("Part 1: build a tracing-subscriber with JSON formatting and EnvFilter")
}

/// Build the main application [`Router`] with all routes and middleware.
pub fn app(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/health/live", get(health::health_live))
        .route("/health/ready", get(health::health_ready))
        .route("/metrics", get(metrics_handler));

    api_routes
        .layer(middleware::metrics_layer())
        .layer(middleware::trace_layer())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(UuidRequestId))
        .with_state(state)
}

/// Handler that renders Prometheus metrics as text.
async fn metrics_handler(State(state): State<AppState>) -> String {
    todo!("Part 3: call state.metrics_handle.render() and return the text")
}
