use std::time::Duration;
use tokio::net::TcpListener;
use tracing::info;

use taskforge_deploy::config::AppConfig;
use taskforge_deploy::feature_flags::FeatureFlagService;
use taskforge_deploy::shutdown::{graceful_shutdown, shutdown_signal};
use taskforge_deploy::{app, AppState};

#[tokio::main]
async fn main() {
    // Initialise structured logging.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Load layered configuration.
    let config = AppConfig::load().expect("failed to load configuration");
    info!(?config, "configuration loaded");

    let shutdown_timeout = Duration::from_secs(config.server.shutdown_timeout_secs);
    let addr = format!("{}:{}", config.server.host, config.server.port);

    // Build shared state.
    let flag_service = FeatureFlagService::new(config.features.clone());
    let state = AppState::new(flag_service);

    // Build the router.
    let router = app(state.clone());

    // Bind the listener.
    let listener = TcpListener::bind(&addr)
        .await
        .expect("failed to bind address");
    info!("listening on {}", addr);

    // Mark the service as ready.
    state.set_ready();
    info!("service is ready");

    // Serve with graceful shutdown.
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");

    // After the server stops accepting, run the drain/flush sequence.
    graceful_shutdown(state, shutdown_timeout).await;
}
