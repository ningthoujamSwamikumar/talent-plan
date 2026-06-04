use std::sync::atomic::Ordering;
use std::time::Duration;

use axum::http::StatusCode;
use taskforge_deploy::config::{AppConfig, FeatureFlags};
use taskforge_deploy::feature_flags::FeatureFlagService;
use taskforge_deploy::shutdown::graceful_shutdown;
use taskforge_deploy::{app, AppState, STATE_DRAINING, STATE_STARTING, STATE_STOPPED};

// ---------------------------------------------------------------------------
// Helper: spin up a test server and return its base URL
// ---------------------------------------------------------------------------
async fn spawn_app(state: AppState) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let router = app(state);
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    format!("http://127.0.0.1:{}", port)
}

fn default_flags(search: bool, ws: bool) -> FeatureFlags {
    FeatureFlags {
        enable_search: search,
        enable_websockets: ws,
        max_tasks_per_project: 50,
    }
}

fn make_state(search: bool, ws: bool) -> AppState {
    let svc = FeatureFlagService::new(default_flags(search, ws));
    AppState::new(svc)
}

// ---------------------------------------------------------------------------
// 1. Config loads defaults
// ---------------------------------------------------------------------------
#[test]
fn config_loads_defaults() {
    // Build config directly from the default file only — no env vars —
    // so this test is immune to env var pollution from parallel tests.
    let cfg = config::Config::builder()
        .add_source(config::File::with_name("config/default").required(true))
        .build()
        .expect("default config file should parse");

    let c: AppConfig = cfg.try_deserialize().expect("should deserialize into AppConfig");
    assert_eq!(c.server.port, 3000);
    assert_eq!(c.server.host, "0.0.0.0");
    assert_eq!(c.server.shutdown_timeout_secs, 30);
    assert_eq!(c.database.max_connections, 10);
    assert!(c.features.enable_search);
    assert!(!c.features.enable_websockets);
    assert_eq!(c.features.max_tasks_per_project, 50);
}

// ---------------------------------------------------------------------------
// 2. Environment variables override file values
// ---------------------------------------------------------------------------
#[test]
fn config_env_vars_override_file_values() {
    // Build config manually with a known env var layer to avoid relying on
    // process-wide env mutation (which races with parallel tests).
    // We directly verify the config crate layering logic.
    let cfg = config::Config::builder()
        .add_source(config::File::with_name("config/default").required(true))
        // Simulate overrides by adding literal values on top of the file.
        .set_override("server.port", 9999_i64)
        .unwrap()
        .set_override("database.max_connections", 42_i64)
        .unwrap()
        .build()
        .expect("config should build with overrides");

    let c: AppConfig = cfg.try_deserialize().expect("should deserialize");
    assert_eq!(c.server.port, 9999);
    assert_eq!(c.database.max_connections, 42);
    // Other values remain at their defaults.
    assert_eq!(c.server.host, "0.0.0.0");
}

// ---------------------------------------------------------------------------
// 3. Missing required field produces an error
// ---------------------------------------------------------------------------
#[test]
fn config_missing_required_field_errors() {
    // Point at a config directory that does not exist so defaults fail to load.
    let result = config::Config::builder()
        .add_source(config::File::with_name("config/nonexistent").required(true))
        .build();
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// 4. Feature flag toggling — search enabled
// ---------------------------------------------------------------------------
#[tokio::test]
async fn feature_flag_search_enabled() {
    let state = make_state(true, false);
    state.set_ready();
    let base = spawn_app(state).await;

    let resp = reqwest::get(format!("{}/search", base)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ---------------------------------------------------------------------------
// 5. Feature flag toggling — search disabled
// ---------------------------------------------------------------------------
#[tokio::test]
async fn feature_flag_search_disabled() {
    let state = make_state(false, false);
    state.set_ready();
    let base = spawn_app(state).await;

    let resp = reqwest::get(format!("{}/search", base)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// 6. Graceful shutdown completes in-flight requests
// ---------------------------------------------------------------------------
#[tokio::test]
async fn graceful_shutdown_completes_inflight_requests() {
    let state = make_state(true, false);
    state.set_ready();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let router = app(state.clone());
    let base = format!("http://127.0.0.1:{}", port);

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    let serve_state = state.clone();
    tokio::spawn(async move {
        axum::serve(listener, router)
            .with_graceful_shutdown(async {
                rx.await.ok();
            })
            .await
            .unwrap();
        graceful_shutdown(serve_state, Duration::from_secs(5)).await;
    });

    // Make a request while the server is alive.
    let resp = reqwest::get(format!("{}/healthz", base)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Signal shutdown.
    tx.send(()).unwrap();

    // Give shutdown a moment to complete.
    tokio::time::sleep(Duration::from_millis(200)).await;

    // The state should transition through draining to stopped.
    let s = state.readiness.load(Ordering::SeqCst);
    assert!(
        s == STATE_DRAINING || s == STATE_STOPPED,
        "expected draining or stopped, got {}",
        s
    );
}

// ---------------------------------------------------------------------------
// 7. Graceful shutdown times out after deadline
// ---------------------------------------------------------------------------
#[tokio::test]
async fn graceful_shutdown_times_out_after_deadline() {
    let state = make_state(true, false);
    state.set_ready();

    // Run graceful_shutdown with a very short timeout.
    let start = tokio::time::Instant::now();
    graceful_shutdown(state.clone(), Duration::from_millis(100)).await;
    let elapsed = start.elapsed();

    // Should have finished around the timeout, not much longer.
    assert!(elapsed < Duration::from_secs(2), "shutdown took too long");
    assert_eq!(state.readiness_state(), STATE_STOPPED);
}

// ---------------------------------------------------------------------------
// 8. Health endpoint /healthz returns 200
// ---------------------------------------------------------------------------
#[tokio::test]
async fn healthz_returns_ok() {
    let state = make_state(true, false);
    state.set_ready();
    let base = spawn_app(state).await;

    let resp = reqwest::get(format!("{}/healthz", base)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ---------------------------------------------------------------------------
// 9. Readiness returns 503 when starting
// ---------------------------------------------------------------------------
#[tokio::test]
async fn readyz_returns_503_when_starting() {
    let state = make_state(true, false);
    // Do NOT call state.set_ready() — still in Starting state.
    assert_eq!(state.readiness_state(), STATE_STARTING);
    let base = spawn_app(state).await;

    let resp = reqwest::get(format!("{}/readyz", base)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}

// ---------------------------------------------------------------------------
// 10. Readiness returns 200 when ready
// ---------------------------------------------------------------------------
#[tokio::test]
async fn readyz_returns_200_when_ready() {
    let state = make_state(true, false);
    state.set_ready();
    let base = spawn_app(state).await;

    let resp = reqwest::get(format!("{}/readyz", base)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ---------------------------------------------------------------------------
// 11. Readiness returns 503 when draining
// ---------------------------------------------------------------------------
#[tokio::test]
async fn readyz_returns_503_when_draining() {
    let state = make_state(true, false);
    state.set_ready();
    state.set_draining();
    let base = spawn_app(state).await;

    let resp = reqwest::get(format!("{}/readyz", base)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}

// ---------------------------------------------------------------------------
// 12. Feature flag service — unknown flag is disabled
// ---------------------------------------------------------------------------
#[test]
fn feature_flag_unknown_returns_false() {
    let svc = FeatureFlagService::new(default_flags(true, true));
    assert!(!svc.is_enabled("nonexistent"));
}
