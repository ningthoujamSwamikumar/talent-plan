use std::time::Duration;
use tokio::time::timeout;
use tracing::info;

use crate::AppState;

/// Returns a future that resolves when a shutdown signal is received.
///
/// Listens for:
/// - `ctrl-c` (SIGINT)
/// - `SIGTERM` (Unix only)
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl-c");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { info!("received ctrl-c"); }
        _ = terminate => { info!("received SIGTERM"); }
    }
}

/// Perform graceful shutdown: mark the app as draining, wait for the
/// specified timeout, then mark as stopped and flush logs.
pub async fn graceful_shutdown(state: AppState, shutdown_timeout: Duration) {
    // 1. Transition to Draining — readiness probes will start returning 503.
    state.set_draining();
    info!("entering graceful shutdown, draining in-flight requests");

    // 2. Give in-flight requests time to complete.
    //    In a real system this would track active connections. Here we simply
    //    sleep for the configured timeout to simulate draining.
    let _ = timeout(shutdown_timeout, async {
        // Placeholder: in production you would wait on an active-request counter
        // reaching zero. For this exercise we just yield for the timeout.
        tokio::time::sleep(shutdown_timeout).await;
    })
    .await;

    // 3. Flush logs / metrics.
    info!("flushing logs and metrics");

    // 4. Mark stopped.
    state.set_stopped();
    info!("shutdown complete");
}
