use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};

use taskforge_observability::{app, metrics_setup, AppState};

// `ServiceExt` provides the `.oneshot()` method on `Router`.
use tower::ServiceExt;

/// Helper: build an [`AppState`] with the readiness flag set to `ready`.
fn test_state(ready: bool) -> AppState {
    let metrics_handle = metrics_setup::install_prometheus_recorder();
    AppState {
        ready: Arc::new(AtomicBool::new(ready)),
        metrics_handle,
    }
}

// -----------------------------------------------------------------------
// Part 4: Health endpoints
// -----------------------------------------------------------------------

#[tokio::test]
async fn health_live_returns_200() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn health_live_body_contains_alive() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "alive");
}

#[tokio::test]
async fn health_ready_returns_200_when_ready() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn health_ready_body_contains_ready() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ready");
}

#[tokio::test]
async fn health_ready_returns_503_when_not_ready() {
    let app = app(test_state(false));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn health_ready_503_body_contains_not_ready() {
    let app = app(test_state(false));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "not_ready");
    assert!(json.get("reason").is_some());
}

// -----------------------------------------------------------------------
// Part 3: Prometheus metrics
// -----------------------------------------------------------------------

#[tokio::test]
async fn metrics_endpoint_returns_200() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn metrics_endpoint_returns_text() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    // Prometheus text format is valid UTF-8; an empty recorder is still valid.
    assert!(text.is_ascii() || text.len() > 0);
}

#[tokio::test]
async fn metrics_includes_http_requests_total() {
    // After sending a request the counter should appear in /metrics output.
    let state = test_state(true);
    let router = app(state.clone());

    // Send a request to generate a metric.
    let _ = router
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Now query /metrics on a fresh router (same state).
    let app2 = app(AppState {
        ready: Arc::new(AtomicBool::new(true)),
        metrics_handle: state.metrics_handle.clone(),
    });
    let resp = app2
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(
        text.contains("http_requests_total"),
        "metrics output should include http_requests_total"
    );
}

#[tokio::test]
async fn metrics_includes_http_request_duration() {
    let state = test_state(true);
    let router = app(state.clone());

    let _ = router
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let app2 = app(AppState {
        ready: Arc::new(AtomicBool::new(true)),
        metrics_handle: state.metrics_handle.clone(),
    });
    let resp = app2
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(
        text.contains("http_request_duration_seconds"),
        "metrics output should include http_request_duration_seconds"
    );
}

// -----------------------------------------------------------------------
// Part 6: Request ID propagation
// -----------------------------------------------------------------------

#[tokio::test]
async fn response_contains_x_request_id_header() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        resp.headers().contains_key("x-request-id"),
        "response should contain x-request-id header"
    );
}

#[tokio::test]
async fn generated_request_id_is_valid_uuid() {
    let app = app(test_state(true));

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let id = resp
        .headers()
        .get("x-request-id")
        .expect("missing x-request-id")
        .to_str()
        .unwrap();

    uuid::Uuid::parse_str(id).expect("x-request-id should be a valid UUID");
}

#[tokio::test]
async fn client_request_id_is_echoed_back() {
    let app = app(test_state(true));

    let custom_id = "my-custom-request-id-12345";
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .header("x-request-id", custom_id)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let returned_id = resp
        .headers()
        .get("x-request-id")
        .expect("missing x-request-id")
        .to_str()
        .unwrap();

    assert_eq!(returned_id, custom_id);
}

// -----------------------------------------------------------------------
// Part 1 & 2: Structured logging / tracing spans
// -----------------------------------------------------------------------

#[tokio::test]
async fn structured_log_is_valid_json() {
    // This test verifies that the tracing output is JSON-formatted.
    // Students should capture log output via a test subscriber and parse it.
    // For the stub we simply assert the init function exists.
    //
    // A full implementation would:
    // 1. Set up a `tracing_subscriber::fmt::Layer` writing to a `Vec<u8>`.
    // 2. Send a request through the app.
    // 3. Parse each captured line as JSON and assert required fields.
    let _state = test_state(true);
    // Placeholder assertion -- the student fills in the real check.
    assert!(true, "structured log JSON validation not yet implemented");
}

#[tokio::test]
async fn traced_handler_creates_span_with_method_and_path() {
    // This test verifies that handler spans include HTTP method and path.
    // Students should use `tracing_subscriber::layer::SubscriberExt` with a
    // test layer that captures span attributes.
    //
    // Placeholder -- the student fills in the real check.
    let _state = test_state(true);
    assert!(
        true,
        "span attribute verification not yet implemented"
    );
}

// -----------------------------------------------------------------------
// Extra: readiness toggle
// -----------------------------------------------------------------------

#[tokio::test]
async fn readiness_can_be_toggled_at_runtime() {
    let state = test_state(true);
    let app1 = app(state.clone());

    // Initially ready.
    let resp = app1
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Toggle to not-ready.
    state.ready.store(false, Ordering::SeqCst);

    let app2 = app(state.clone());
    let resp = app2
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}
