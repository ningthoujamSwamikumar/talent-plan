//! Tests for the API Gateway (Part 2).
//!
//! These tests start the gateway server and exercise it via HTTP.

use reqwest::Client;
use serde_json::Value;
use std::net::TcpListener;
use system_design::api_gateway::{GatewayConfig, RateLimiter, RouteRule};

/// Start a gateway with the given config and return its base URL.
async fn start_gateway(config: GatewayConfig) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let addr = format!("127.0.0.1:{port}");
    let base_url = format!("http://{addr}");

    let router = system_design::api_gateway::gateway_router(config);
    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tokio::spawn(async move {
        axum::serve(tcp_listener, router).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    base_url
}

fn default_config() -> GatewayConfig {
    GatewayConfig::new(
        vec![
            RouteRule {
                prefix: "/api/users".to_string(),
                backend_url: "http://users-service:3001".to_string(),
            },
            RouteRule {
                prefix: "/api/orders".to_string(),
                backend_url: "http://orders-service:3002".to_string(),
            },
        ],
        RateLimiter::new(10, 1.0),
    )
}

fn client() -> Client {
    Client::builder().build().unwrap()
}

/// Test 1: route_to_correct_backend
#[tokio::test]
async fn test_route_to_correct_backend() {
    let base = start_gateway(default_config()).await;
    let resp: Value = client()
        .get(format!("{base}/api/users/123"))
        .header("x-api-key", "client-a")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(resp["backend"], "http://users-service:3001");
    assert_eq!(resp["path"], "/api/users/123");
}

/// Test 2: unknown_route_returns_404
#[tokio::test]
async fn test_unknown_route_returns_404() {
    let base = start_gateway(default_config()).await;
    let resp = client()
        .get(format!("{base}/unknown/path"))
        .header("x-api-key", "client-b")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}

/// Test 3: rate_limit_allows_within_limit
#[tokio::test]
async fn test_rate_limit_allows_within_limit() {
    let config = GatewayConfig::new(
        vec![RouteRule {
            prefix: "/api".to_string(),
            backend_url: "http://backend:3000".to_string(),
        }],
        RateLimiter::new(5, 0.0), // 5 tokens, no refill
    );
    let base = start_gateway(config).await;
    let c = client();

    // All 5 requests should succeed.
    for _ in 0..5 {
        let resp = c
            .get(format!("{base}/api/test"))
            .header("x-api-key", "client-c")
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
    }
}

/// Test 4: rate_limit_rejects_over_limit
#[tokio::test]
async fn test_rate_limit_rejects_over_limit() {
    let config = GatewayConfig::new(
        vec![RouteRule {
            prefix: "/api".to_string(),
            backend_url: "http://backend:3000".to_string(),
        }],
        RateLimiter::new(3, 0.0), // 3 tokens, no refill
    );
    let base = start_gateway(config).await;
    let c = client();

    // Exhaust the 3 tokens.
    for _ in 0..3 {
        let resp = c
            .get(format!("{base}/api/test"))
            .header("x-api-key", "client-d")
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
    }

    // 4th request should be rate limited.
    let resp = c
        .get(format!("{base}/api/test"))
        .header("x-api-key", "client-d")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 429);
}

/// Test 5: rate_limit_per_client_ip
#[tokio::test]
async fn test_rate_limit_per_client_ip() {
    let config = GatewayConfig::new(
        vec![RouteRule {
            prefix: "/api".to_string(),
            backend_url: "http://backend:3000".to_string(),
        }],
        RateLimiter::new(2, 0.0), // 2 tokens, no refill
    );
    let base = start_gateway(config).await;
    let c = client();

    // Exhaust client-e's tokens.
    for _ in 0..2 {
        c.get(format!("{base}/api/test"))
            .header("x-api-key", "client-e")
            .send()
            .await
            .unwrap();
    }
    let resp = c
        .get(format!("{base}/api/test"))
        .header("x-api-key", "client-e")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 429);

    // client-f should still have tokens.
    let resp = c
        .get(format!("{base}/api/test"))
        .header("x-api-key", "client-f")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

/// Test 6: rate_limit_refills_over_time
#[tokio::test]
async fn test_rate_limit_refills_over_time() {
    let config = GatewayConfig::new(
        vec![RouteRule {
            prefix: "/api".to_string(),
            backend_url: "http://backend:3000".to_string(),
        }],
        RateLimiter::new(1, 10.0), // 1 token, fast refill (10/sec)
    );
    let base = start_gateway(config).await;
    let c = client();

    // Use the one token.
    let resp = c
        .get(format!("{base}/api/test"))
        .header("x-api-key", "client-g")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Immediately should be rate-limited.
    let resp = c
        .get(format!("{base}/api/test"))
        .header("x-api-key", "client-g")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 429);

    // Wait for refill.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Should have tokens again.
    let resp = c
        .get(format!("{base}/api/test"))
        .header("x-api-key", "client-g")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

/// Test 7: multiple_routes_work
#[tokio::test]
async fn test_multiple_routes_work() {
    let base = start_gateway(default_config()).await;
    let c = client();

    let users_resp: Value = c
        .get(format!("{base}/api/users/1"))
        .header("x-api-key", "client-h")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(users_resp["backend"], "http://users-service:3001");

    let orders_resp: Value = c
        .get(format!("{base}/api/orders/42"))
        .header("x-api-key", "client-h")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(orders_resp["backend"], "http://orders-service:3002");
}

/// Test 8: gateway_forwards_headers
#[tokio::test]
async fn test_gateway_forwards_headers() {
    let base = start_gateway(default_config()).await;
    let resp: Value = client()
        .get(format!("{base}/api/users/me"))
        .header("x-api-key", "client-i")
        .header("authorization", "Bearer token123")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let headers = resp["headers"].as_array().unwrap();
    let header_strs: Vec<String> = headers
        .iter()
        .map(|h| h.as_str().unwrap().to_string())
        .collect();
    // The authorization header should be forwarded.
    assert!(header_strs
        .iter()
        .any(|h| h.contains("authorization: Bearer token123")));
}

/// Test 9: gateway_forwards_body
#[tokio::test]
async fn test_gateway_forwards_body() {
    let base = start_gateway(default_config()).await;
    let resp: Value = client()
        .post(format!("{base}/api/users"))
        .header("x-api-key", "client-j")
        .header("content-type", "application/json")
        .body(r#"{"name":"alice"}"#)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(resp["body"], r#"{"name":"alice"}"#);
    assert_eq!(resp["method"], "POST");
}

/// Test 10: gateway_handles_backend_error
#[tokio::test]
async fn test_gateway_handles_backend_error() {
    // When no route matches, the gateway returns 404.
    let config = GatewayConfig::new(
        vec![], // No routes registered
        RateLimiter::new(10, 1.0),
    );
    let base = start_gateway(config).await;

    let resp = client()
        .get(format!("{base}/api/anything"))
        .header("x-api-key", "client-k")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}
