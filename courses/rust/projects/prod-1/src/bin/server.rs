use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use taskforge_observability::{app, init_tracing, metrics_setup, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Part 1: initialise structured logging.
    init_tracing();

    // Part 3: install the Prometheus metrics recorder.
    let metrics_handle = metrics_setup::install_prometheus_recorder();

    let state = AppState {
        ready: Arc::new(AtomicBool::new(true)),
        metrics_handle,
    };

    let app = app(state);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");
}
