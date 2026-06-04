//! Part 1: URL Shortener
//!
//! # Design Document
//!
//! ## Capacity Estimation
//! - 100M URLs/month = ~40 URLs/sec writes
//! - 10:1 read ratio = ~400 redirects/sec reads
//! - Each URL record: ~500 bytes (short code + long URL + metadata)
//! - 5-year storage: 100M * 12 * 5 * 500B = ~3TB
//!
//! ## API
//! - POST /shorten { "url": "https://..." } -> { "short_code": "abc123", "short_url": "..." }
//! - GET /:code -> 302 Redirect to original URL
//! - GET /:code/stats -> { "visits": N, "created_at": "...", "original_url": "..." }
//!
//! ## Data Model
//! - ShortenedUrl: { code, original_url, visits, created_at }
//! - Short code: 7-char base62 string (62^7 = 3.5 trillion combinations)
//! - Primary lookup index on `code`
//!
//! ## Architecture (production)
//! - Application servers behind a load balancer
//! - Redis cache for hot short codes (>90% cache hit rate expected)
//! - PostgreSQL for persistent storage
//! - Counter-based or random code generation with collision retry
//!
//! For this implementation: single-node, in-memory HashMap.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

/// A shortened URL record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenedUrl {
    /// The generated short code.
    pub code: String,
    /// The original long URL.
    pub original_url: String,
    /// Number of times the short URL has been visited.
    pub visits: u64,
    /// When the short URL was created.
    pub created_at: DateTime<Utc>,
}

/// Request to shorten a URL.
#[derive(Debug, Clone, Deserialize)]
pub struct ShortenRequest {
    /// The URL to shorten.
    pub url: String,
}

/// Response after shortening.
#[derive(Debug, Clone, Serialize)]
pub struct ShortenResponse {
    /// The generated short code.
    pub short_code: String,
    /// The full short URL (base_url + code).
    pub short_url: String,
    /// The original URL that was shortened.
    pub original_url: String,
}

/// Stats response for a short code.
#[derive(Debug, Clone, Serialize)]
pub struct StatsResponse {
    /// The short code.
    pub code: String,
    /// The original URL.
    pub original_url: String,
    /// Number of visits.
    pub visits: u64,
    /// When the URL was created.
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// URL Shortener core
// ---------------------------------------------------------------------------

/// Characters used for base62 encoding.
const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// In-memory URL shortener.
#[derive(Debug)]
pub struct UrlShortener {
    /// Map from short code to URL record.
    urls: Arc<RwLock<HashMap<String, ShortenedUrl>>>,
    /// Map from original URL to short code (for dedup).
    reverse: Arc<RwLock<HashMap<String, String>>>,
    /// Monotonic counter for code generation.
    counter: AtomicU64,
}

impl UrlShortener {
    /// Creates a new, empty URL shortener.
    pub fn new() -> Self {
        Self {
            urls: Arc::new(RwLock::new(HashMap::new())),
            reverse: Arc::new(RwLock::new(HashMap::new())),
            counter: AtomicU64::new(1),
        }
    }

    /// Generate a short code using base62 encoding of the counter.
    ///
    /// Encodes the current counter value in base62 (0-9, A-Z, a-z)
    /// and pads to at least 7 characters.
    pub fn generate_code(&self) -> String {
        let mut n = self.counter.fetch_add(1, Ordering::Relaxed);
        let base = BASE62_CHARS.len() as u64; // 62
        let mut code = Vec::new();

        if n == 0 {
            code.push(BASE62_CHARS[0]);
        } else {
            while n > 0 {
                let remainder = (n % base) as usize;
                code.push(BASE62_CHARS[remainder]);
                n /= base;
            }
        }

        // Reverse to get most-significant digit first.
        code.reverse();

        // Pad to at least 7 characters with leading '0's.
        let mut result = String::from_utf8(code).unwrap();
        while result.len() < 7 {
            result.insert(0, '0');
        }

        result
    }

    /// Shorten a URL. If the URL was already shortened, return the existing code.
    pub async fn shorten(&self, original_url: &str) -> Result<ShortenedUrl, ShortenerError> {
        if original_url.is_empty() {
            return Err(ShortenerError::InvalidUrl);
        }

        // Check reverse map for duplicates.
        {
            let reverse = self.reverse.read().await;
            if let Some(existing_code) = reverse.get(original_url) {
                let urls = self.urls.read().await;
                if let Some(record) = urls.get(existing_code) {
                    return Ok(record.clone());
                }
            }
        }

        let code = self.generate_code();
        let record = ShortenedUrl {
            code: code.clone(),
            original_url: original_url.to_string(),
            visits: 0,
            created_at: Utc::now(),
        };

        {
            let mut urls = self.urls.write().await;
            urls.insert(code.clone(), record.clone());
        }
        {
            let mut reverse = self.reverse.write().await;
            reverse.insert(original_url.to_string(), code);
        }

        Ok(record)
    }

    /// Look up a short code and return the original URL. Increments the visit count.
    pub async fn resolve(&self, code: &str) -> Result<ShortenedUrl, ShortenerError> {
        let mut urls = self.urls.write().await;
        let record = urls.get_mut(code).ok_or(ShortenerError::NotFound)?;
        record.visits += 1;
        Ok(record.clone())
    }

    /// Get stats for a short code without incrementing the visit count.
    pub async fn stats(&self, code: &str) -> Result<StatsResponse, ShortenerError> {
        let urls = self.urls.read().await;
        let record = urls.get(code).ok_or(ShortenerError::NotFound)?;
        Ok(StatsResponse {
            code: record.code.clone(),
            original_url: record.original_url.clone(),
            visits: record.visits,
            created_at: record.created_at,
        })
    }
}

impl Default for UrlShortener {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors from the URL shortener.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortenerError {
    /// The provided URL is empty or invalid.
    InvalidUrl,
    /// The short code was not found.
    NotFound,
}

impl std::fmt::Display for ShortenerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUrl => write!(f, "Invalid or empty URL"),
            Self::NotFound => write!(f, "Short code not found"),
        }
    }
}

impl std::error::Error for ShortenerError {}

// ---------------------------------------------------------------------------
// Axum handlers
// ---------------------------------------------------------------------------

/// Shared state for the shortener service.
#[derive(Clone)]
pub struct ShortenerState {
    /// The URL shortener instance.
    pub shortener: Arc<UrlShortener>,
    /// The base URL for generating short URLs (e.g., "http://localhost:3000").
    pub base_url: String,
}

/// POST /shorten -- create a new short URL.
pub async fn shorten_handler(
    State(state): State<ShortenerState>,
    Json(body): Json<ShortenRequest>,
) -> Result<(StatusCode, Json<ShortenResponse>), StatusCode> {
    let record = state
        .shortener
        .shorten(&body.url)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let response = ShortenResponse {
        short_code: record.code.clone(),
        short_url: format!("{}/{}", state.base_url, record.code),
        original_url: record.original_url,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// GET /:code -- redirect to the original URL.
pub async fn redirect_handler(
    State(state): State<ShortenerState>,
    Path(code): Path<String>,
) -> Result<(StatusCode, [(&'static str, String); 1]), StatusCode> {
    let record = state
        .shortener
        .resolve(&code)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((
        StatusCode::FOUND,
        [("location", record.original_url)],
    ))
}

/// GET /:code/stats -- return visit count and metadata.
pub async fn stats_handler(
    State(state): State<ShortenerState>,
    Path(code): Path<String>,
) -> Result<Json<StatsResponse>, StatusCode> {
    let stats = state
        .shortener
        .stats(&code)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(stats))
}

/// Build the URL shortener router.
pub fn shortener_router(base_url: &str) -> axum::Router {
    let state = ShortenerState {
        shortener: Arc::new(UrlShortener::new()),
        base_url: base_url.to_string(),
    };

    axum::Router::new()
        .route("/shorten", post(shorten_handler))
        .route("/:code/stats", get(stats_handler))
        .route("/:code", get(redirect_handler))
        .with_state(state)
}
