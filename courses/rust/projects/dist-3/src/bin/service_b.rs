//! Service B — "Notifications" service.
//!
//! Accepts notification requests triggered by other services. Registers itself
//! with the service registry and sends periodic heartbeats.

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", get(|| async { "ok" }));

    // TODO:
    // 1. Register with ServiceRegistry.
    // 2. Spawn heartbeat task.
    // 3. Add notification routes.
    // 4. Apply idempotency and tracing middleware.

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    tracing::info!("service_b listening on 3002");
    axum::serve(listener, app).await.unwrap();
}
