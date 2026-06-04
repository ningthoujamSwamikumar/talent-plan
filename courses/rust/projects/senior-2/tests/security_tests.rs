//! Integration tests for Security Hardening.
//!
//! These tests start the server and exercise security features via HTTP.
//! All assertions use JSON values — no imports from the crate needed.

use reqwest::Client;
use serde_json::{json, Value};
use std::net::TcpListener;

const TEST_HMAC_KEY: &str = "integration-test-hmac-key";

/// Start the server on a random port and return the base URL.
async fn start_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let addr = format!("127.0.0.1:{port}");
    let base_url = format!("http://{addr}");
    let hmac_key = TEST_HMAC_KEY.to_string();

    tokio::spawn(async move {
        security_hardening::run_server(&addr, &hmac_key).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    base_url
}

fn client() -> Client {
    Client::new()
}

// =========================================================================
// Part 1: Input Sanitization
// =========================================================================

#[tokio::test]
async fn test_xss_in_title_is_stripped() {
    let base = start_server().await;
    let note: Value = client()
        .post(format!("{base}/notes"))
        .json(&json!({
            "title": "<script>alert('xss')</script>My Note",
            "content": "Safe content",
            "author": "tester"
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let title = note["title"].as_str().unwrap();
    assert!(!title.contains("<script>"), "Script tag should be stripped from title");
    assert!(title.contains("My Note"));
}

#[tokio::test]
async fn test_xss_in_content_is_stripped() {
    let base = start_server().await;
    let note: Value = client()
        .post(format!("{base}/notes"))
        .json(&json!({
            "title": "Clean Title",
            "content": "<p>Good</p><iframe src='evil.com'></iframe><img onerror='alert(1)'>",
            "author": "tester"
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let content = note["content"].as_str().unwrap();
    assert!(content.contains("<p>Good</p>"));
    assert!(!content.contains("<iframe"));
    assert!(!content.contains("onerror"));
}

// =========================================================================
// Part 3: Security Headers
// =========================================================================

#[tokio::test]
async fn test_hsts_header_present() {
    let base = start_server().await;
    let resp = client().get(format!("{base}/notes")).send().await.unwrap();
    let hsts = resp.headers().get("strict-transport-security");
    assert!(hsts.is_some(), "HSTS header should be present");
    assert!(hsts.unwrap().to_str().unwrap().contains("max-age=63072000"));
}

#[tokio::test]
async fn test_csp_header_present() {
    let base = start_server().await;
    let resp = client().get(format!("{base}/notes")).send().await.unwrap();
    let csp = resp.headers().get("content-security-policy");
    assert!(csp.is_some(), "CSP header should be present");
    assert!(csp.unwrap().to_str().unwrap().contains("default-src 'self'"));
}

#[tokio::test]
async fn test_x_content_type_options_header() {
    let base = start_server().await;
    let resp = client().get(format!("{base}/notes")).send().await.unwrap();
    let header = resp.headers().get("x-content-type-options");
    assert!(header.is_some());
    assert_eq!(header.unwrap().to_str().unwrap(), "nosniff");
}

#[tokio::test]
async fn test_x_frame_options_header() {
    let base = start_server().await;
    let resp = client().get(format!("{base}/notes")).send().await.unwrap();
    let header = resp.headers().get("x-frame-options");
    assert!(header.is_some());
    assert_eq!(header.unwrap().to_str().unwrap(), "DENY");
}

#[tokio::test]
async fn test_referrer_policy_header() {
    let base = start_server().await;
    let resp = client().get(format!("{base}/notes")).send().await.unwrap();
    let header = resp.headers().get("referrer-policy");
    assert!(header.is_some());
    assert_eq!(
        header.unwrap().to_str().unwrap(),
        "strict-origin-when-cross-origin"
    );
}

// =========================================================================
// Part 4: HMAC Authentication (integration)
// =========================================================================

#[tokio::test]
async fn test_missing_signature_returns_401() {
    let base = start_server().await;
    // POST without HMAC headers should be rejected
    let resp = client()
        .post(format!("{base}/notes"))
        .json(&json!({
            "title": "Unsigned",
            "content": "No signature",
            "author": "anon"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401, "Unsigned mutation should return 401");
}

#[tokio::test]
async fn test_invalid_signature_returns_401() {
    let base = start_server().await;
    let resp = client()
        .post(format!("{base}/notes"))
        .header("X-Signature", "definitely-wrong")
        .header("X-Timestamp", "1700000000")
        .json(&json!({
            "title": "Bad Sig",
            "content": "Wrong signature",
            "author": "anon"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401, "Invalid signature should return 401");
}

// =========================================================================
// Part 5: Audit Logging (integration)
// =========================================================================

#[tokio::test]
async fn test_read_operations_do_not_require_hmac() {
    let base = start_server().await;
    // GET (read) should NOT require HMAC authentication
    let resp = client().get(format!("{base}/notes")).send().await.unwrap();
    assert_eq!(resp.status(), 200, "Read operations should not require HMAC");
}

// =========================================================================
// Part 6: Secrets Management (unit-level, no server needed)
// =========================================================================

#[test]
fn test_secrets_debug_is_redacted() {
    let secrets = security_hardening::secrets::Secrets::from_values(
        "super-secret".to_string(),
        "postgres://localhost/db".to_string(),
        "api-key-xyz".to_string(),
    );
    let debug = format!("{:?}", secrets);
    assert!(!debug.contains("super-secret"));
    assert!(!debug.contains("postgres://"));
    assert!(!debug.contains("api-key-xyz"));
}

#[test]
fn test_secrets_scrubbing() {
    let secrets = security_hardening::secrets::Secrets::from_values(
        "hmac-key-value".to_string(),
        "postgres://user:pass@host/db".to_string(),
        "sk-live-token".to_string(),
    );
    let msg = "Error connecting to postgres://user:pass@host/db";
    let scrubbed = secrets.scrub_secrets(msg);
    assert!(!scrubbed.contains("user:pass@host"));
    assert!(scrubbed.contains("[REDACTED]"));
}

// =========================================================================
// CRUD basics (through security layers)
// =========================================================================

#[tokio::test]
async fn test_full_crud_through_security_layers() {
    let base = start_server().await;
    let c = client();

    // List (no auth required) should return empty
    let list: Vec<Value> = c
        .get(format!("{base}/notes"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(list.is_empty());

    // Note: Creating/updating/deleting through the HMAC layer requires
    // properly signed requests. Students need to implement the signing
    // client-side using the `sign_request` function.
}
