//! Service C — "Search" service.
//!
//! Accepts indexing requests so that tasks are searchable. Registers itself
//! with the service registry and sends periodic heartbeats.

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", get(|| async { "ok" }));

    // TODO:
    // 1. Register with ServiceRegistry.
    // 2. Spawn heartbeat task.
    // 3. Add search/index routes.
    // 4. Apply idempotency and tracing middleware.

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    tracing::info!("service_c listening on 3003");
    axum::serve(listener, app).await.unwrap();
}
