//! Tests for the URL Shortener (Part 1).
//!
//! These tests start the shortener server and exercise it via HTTP.

use reqwest::Client;
use serde_json::{json, Value};
use std::net::TcpListener;

async fn start_shortener() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let addr = format!("127.0.0.1:{port}");
    let base_url = format!("http://{addr}");

    let router = system_design::url_shortener::shortener_router(&base_url);
    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tokio::spawn(async move {
        axum::serve(tcp_listener, router).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    base_url
}

fn client() -> Client {
    Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

/// Test 1: shorten_returns_code
#[tokio::test]
async fn test_shorten_returns_code() {
    let base = start_shortener().await;
    let resp: Value = client()
        .post(format!("{base}/shorten"))
        .json(&json!({"url": "https://www.example.com/very/long/path"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert!(resp.get("short_code").is_some());
    assert!(resp.get("short_url").is_some());
    assert_eq!(resp["original_url"], "https://www.example.com/very/long/path");
}

/// Test 2: resolve_returns_original_url
#[tokio::test]
async fn test_resolve_returns_original_url() {
    let base = start_shortener().await;
    let c = client();

    let created: Value = c
        .post(format!("{base}/shorten"))
        .json(&json!({"url": "https://example.com/redirect-test"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let code = created["short_code"].as_str().unwrap();

    let resp = c.get(format!("{base}/{code}")).send().await.unwrap();
    assert_eq!(resp.status(), 302);
    let location = resp.headers().get("location").unwrap().to_str().unwrap();
    assert_eq!(location, "https://example.com/redirect-test");
}

/// Test 3: shorten_same_url_returns_same_code
#[tokio::test]
async fn test_shorten_same_url_returns_same_code() {
    let base = start_shortener().await;
    let c = client();

    let first: Value = c
        .post(format!("{base}/shorten"))
        .json(&json!({"url": "https://example.com/dedup"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let second: Value = c
        .post(format!("{base}/shorten"))
        .json(&json!({"url": "https://example.com/dedup"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(first["short_code"], second["short_code"]);
}

/// Test 4: resolve_unknown_code_returns_none
#[tokio::test]
async fn test_resolve_unknown_code_returns_none() {
    let base = start_shortener().await;
    let resp = client()
        .get(format!("{base}/nonexistent"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

/// Test 5: click_count_starts_at_zero
#[tokio::test]
async fn test_click_count_starts_at_zero() {
    let base = start_shortener().await;
    let c = client();

    let created: Value = c
        .post(format!("{base}/shorten"))
        .json(&json!({"url": "https://example.com/no-visits"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let code = created["short_code"].as_str().unwrap();

    let stats: Value = c
        .get(format!("{base}/{code}/stats"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(stats["visits"], 0);
}

/// Test 6: click_count_increments_on_resolve
#[tokio::test]
async fn test_click_count_increments_on_resolve() {
    let base = start_shortener().await;
    let c = client();

    let created: Value = c
        .post(format!("{base}/shorten"))
        .json(&json!({"url": "https://example.com/stats-test"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let code = created["short_code"].as_str().unwrap();

    // Visit the URL twice.
    c.get(format!("{base}/{code}")).send().await.unwrap();
    c.get(format!("{base}/{code}")).send().await.unwrap();

    let stats: Value = c
        .get(format!("{base}/{code}/stats"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(stats["visits"], 2);
    assert!(stats.get("created_at").is_some());
}

/// Test 7: shorten_with_empty_url_returns_error (substitute for TTL test at HTTP level)
#[tokio::test]
async fn test_shorten_with_empty_url_returns_error() {
    let base = start_shortener().await;
    let resp = client()
        .post(format!("{base}/shorten"))
        .json(&json!({"url": ""}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

/// Test 8: stats_for_nonexistent_code_returns_404
#[tokio::test]
async fn test_stats_for_nonexistent_code_returns_404() {
    let base = start_shortener().await;
    let resp = client()
        .get(format!("{base}/doesnotexist/stats"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

/// Test 9: concurrent_shortening_is_safe
#[tokio::test]
async fn test_concurrent_shortening_is_safe() {
    let base = start_shortener().await;
    let c = client();

    let mut handles = Vec::new();
    for i in 0..10 {
        let url = format!("{base}/shorten");
        let client = c.clone();
        let body = json!({"url": format!("https://example.com/concurrent/{i}")});
        handles.push(tokio::spawn(async move {
            let resp: Value = client
                .post(url)
                .json(&body)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            resp["short_code"].as_str().unwrap().to_string()
        }));
    }

    let mut codes = Vec::new();
    for h in handles {
        codes.push(h.await.unwrap());
    }

    // All codes should be unique.
    codes.sort();
    codes.dedup();
    assert_eq!(codes.len(), 10);
}

/// Test 10: code_is_url_safe_characters
#[tokio::test]
async fn test_code_is_url_safe_characters() {
    let base = start_shortener().await;
    let resp: Value = client()
        .post(format!("{base}/shorten"))
        .json(&json!({"url": "https://example.com/format"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let code = resp["short_code"].as_str().unwrap();
    // Code should be alphanumeric (base62) and reasonable length.
    assert!(!code.is_empty() && code.len() <= 12);
    assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
}
