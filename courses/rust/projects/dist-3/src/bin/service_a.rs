//! Service A — "Tasks" service.
//!
//! Provides CRUD operations for tasks. Registers itself with the service
//! registry, sends periodic heartbeats, and participates in the "create task"
//! saga.

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", get(|| async { "ok" }));

    // TODO:
    // 1. Register with ServiceRegistry.
    // 2. Spawn heartbeat task.
    // 3. Add task CRUD routes.
    // 4. Apply idempotency and tracing middleware.

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("service_a listening on 3001");
    axum::serve(listener, app).await.unwrap();
}
