use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::sync::mpsc;
use tracing_subscriber;

use realtime_service::handlers::{ws_handler, AppState};
use realtime_service::heartbeat::spawn_disconnect_monitor;
use realtime_service::manager::ConnectionManager;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let manager = Arc::new(ConnectionManager::new());

    let (disconnect_tx, disconnect_rx) = mpsc::channel::<String>(256);

    // Spawn the global disconnect monitor.
    spawn_disconnect_monitor(Arc::clone(&manager), disconnect_rx);

    let state = AppState {
        manager,
        disconnect_tx,
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");
}
