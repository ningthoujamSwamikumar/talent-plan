//! Binary entry point for the TaskForge API server.

use std::sync::Arc;
use taskforge_api::{app, store::AppState};

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber so log/trace output appears on stdout.
    tracing_subscriber::fmt::init();

    // Create shared application state.
    let state = Arc::new(AppState::new());

    // Build the router.
    let app = app(state);

    // Bind to localhost:3000 and serve.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
