//! Part 2: Rate-Limited API Gateway
//!
//! # Design Document
//!
//! ## Requirements
//! - Route requests to different backends based on URL prefix
//! - Apply per-client rate limits using token bucket algorithm
//! - Forward headers and request body
//! - Return 429 Too Many Requests when limit exceeded
//!
//! ## Rate Limiting: Token Bucket
//! - Each client gets a bucket with `capacity` tokens
//! - Tokens refill at `refill_rate` per second
//! - Each request consumes one token
//! - If bucket is empty, request is rejected with 429
//! - Client identified by `X-API-Key` header or IP address
//!
//! ## Architecture
//! - Middleware pipeline: identify client -> check rate limit -> route -> forward
//! - Horizontal scaling: use Redis for shared rate limit state
//! - For this implementation: in-memory DashMap per instance

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ---------------------------------------------------------------------------
// Token Bucket
// ---------------------------------------------------------------------------

/// A token bucket for rate limiting.
///
/// Tokens refill continuously at a fixed rate. Each request consumes one token.
/// When no tokens remain, requests are rejected.
#[derive(Debug, Clone)]
pub struct TokenBucket {
    /// Maximum number of tokens.
    pub capacity: u64,
    /// Current number of available tokens.
    pub tokens: f64,
    /// Tokens added per second.
    pub refill_rate: f64,
    /// Last time tokens were refilled.
    pub last_refill: DateTime<Utc>,
}

impl TokenBucket {
    /// Create a new token bucket, starting full.
    pub fn new(capacity: u64, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate,
            last_refill: Utc::now(),
        }
    }

    /// Try to acquire one token. Returns `true` if successful, `false` if empty.
    ///
    /// Automatically refills tokens based on elapsed time before checking.
    pub fn try_acquire(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Refill tokens based on elapsed time since last refill.
    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed = (now - self.last_refill).num_milliseconds().max(0) as f64 / 1000.0;
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity as f64);
        self.last_refill = now;
    }

    /// How many seconds until the next token is available?
    /// Returns 0.0 if tokens are already available.
    pub fn retry_after(&self) -> f64 {
        if self.tokens >= 1.0 {
            0.0
        } else {
            (1.0 - self.tokens) / self.refill_rate
        }
    }
}

// ---------------------------------------------------------------------------
// Rate Limiter
// ---------------------------------------------------------------------------

/// Per-client rate limiter backed by a map of token buckets.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Map from client ID to their token bucket.
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    /// Default bucket capacity for new clients.
    pub default_capacity: u64,
    /// Default refill rate for new clients.
    pub default_refill_rate: f64,
}

impl RateLimiter {
    /// Create a new rate limiter with default settings.
    pub fn new(default_capacity: u64, default_refill_rate: f64) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            default_capacity,
            default_refill_rate,
        }
    }

    /// Check if a client is allowed to make a request.
    ///
    /// Looks up (or creates) the client's bucket and tries to acquire a token.
    /// Returns `Ok(())` if allowed, `Err(retry_after_seconds)` if rate-limited.
    pub async fn check_rate_limit(&self, client_id: &str) -> Result<(), f64> {
        let mut buckets = self.buckets.write().await;
        let bucket = buckets
            .entry(client_id.to_string())
            .or_insert_with(|| TokenBucket::new(self.default_capacity, self.default_refill_rate));

        if bucket.try_acquire() {
            Ok(())
        } else {
            Err(bucket.retry_after())
        }
    }

    /// Remove buckets that have been full (idle) for longer than `max_idle` seconds.
    pub async fn cleanup_idle(&self, max_idle_secs: i64) {
        let now = Utc::now();
        let mut buckets = self.buckets.write().await;
        buckets.retain(|_, bucket| {
            let idle_secs = (now - bucket.last_refill).num_seconds();
            // Keep buckets that are not full or have been used recently.
            bucket.tokens < bucket.capacity as f64 || idle_secs < max_idle_secs
        });
    }
}

// ---------------------------------------------------------------------------
// Gateway Configuration
// ---------------------------------------------------------------------------

/// A routing rule: requests matching `prefix` are forwarded to `backend_url`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRule {
    /// URL path prefix to match (e.g., "/api/users").
    pub prefix: String,
    /// Backend URL to forward to (e.g., "http://localhost:3001").
    pub backend_url: String,
}

/// Gateway configuration.
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    /// Routing rules, checked in order (longest prefix match wins).
    pub routes: Vec<RouteRule>,
    /// Rate limiter.
    pub rate_limiter: RateLimiter,
}

impl GatewayConfig {
    /// Create a new gateway config.
    pub fn new(routes: Vec<RouteRule>, rate_limiter: RateLimiter) -> Self {
        Self {
            routes,
            rate_limiter,
        }
    }

    /// Find the backend URL for a given request path.
    ///
    /// Returns the backend URL of the route with the longest matching prefix.
    pub fn resolve_backend(&self, path: &str) -> Option<&str> {
        let mut best: Option<(usize, &str)> = None;
        for rule in &self.routes {
            if path.starts_with(&rule.prefix) {
                let len = rule.prefix.len();
                if best.as_ref().map_or(true, |(best_len, _)| len > *best_len) {
                    best = Some((len, &rule.backend_url));
                }
            }
        }
        best.map(|(_, url)| url)
    }
}

// ---------------------------------------------------------------------------
// Gateway handlers
// ---------------------------------------------------------------------------

/// Build the API gateway router.
///
/// Creates a catch-all handler that:
/// 1. Extracts the client ID from `X-API-Key` header (or uses "anonymous")
/// 2. Checks the rate limiter
/// 3. If rate-limited, returns 429 with `Retry-After` header
/// 4. Otherwise, resolves the backend and forwards the request
/// 5. If no backend matches, returns 404
pub fn gateway_router(config: GatewayConfig) -> axum::Router {
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::{Response, StatusCode};
    use axum::routing::any;

    let shared_config = Arc::new(config);

    let handler = move |req: Request<Body>| {
        let config = shared_config.clone();
        async move {
            // Extract client ID from X-API-Key header.
            let client_id = req
                .headers()
                .get("x-api-key")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("anonymous")
                .to_string();

            let path = req.uri().path().to_string();
            let method = req.method().to_string();

            // Collect headers for forwarding.
            let headers: Vec<(String, String)> = req
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            // Read body.
            let body_bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024)
                .await
                .unwrap_or_default();
            let body_str = String::from_utf8_lossy(&body_bytes).to_string();

            // Check rate limit.
            if let Err(retry_after) = config.rate_limiter.check_rate_limit(&client_id).await {
                let retry_secs = retry_after.ceil() as u64;
                return Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .header("retry-after", retry_secs.to_string())
                    .body(Body::from("Too Many Requests"))
                    .unwrap();
            }

            // Resolve backend.
            let backend = match config.resolve_backend(&path) {
                Some(b) => b.to_string(),
                None => {
                    return Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Not Found"))
                        .unwrap();
                }
            };

            // Build a simulated forwarded response (in production, use reqwest).
            let forwarded_headers: Vec<String> =
                headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();

            let response_body = serde_json::json!({
                "backend": backend,
                "method": method,
                "path": path,
                "headers": forwarded_headers,
                "body": body_str,
            });

            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .header("x-backend", &backend)
                .body(Body::from(response_body.to_string()))
                .unwrap()
        }
    };

    axum::Router::new().fallback(any(handler))
}
