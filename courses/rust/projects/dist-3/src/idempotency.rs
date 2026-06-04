use axum::{
    extract::Request,
    http::{HeaderName, StatusCode},
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use std::sync::Arc;

/// Header name for idempotency keys.
pub static IDEMPOTENCY_KEY_HEADER: HeaderName = HeaderName::from_static("idempotency-key");

/// Cached response data stored for idempotent replay.
#[derive(Debug, Clone)]
pub struct CachedResponse {
    pub status: StatusCode,
    pub body: Vec<u8>,
}

/// Shared store of idempotency keys to cached responses.
#[derive(Debug, Clone)]
pub struct IdempotencyStore {
    cache: Arc<DashMap<String, CachedResponse>>,
}

impl IdempotencyStore {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Look up a cached response for the given key.
    pub fn get(&self, key: &str) -> Option<CachedResponse> {
        self.cache.get(key).map(|entry| entry.clone())
    }

    /// Store a response for the given key.
    pub fn insert(&self, key: String, response: CachedResponse) {
        self.cache.insert(key, response);
    }
}

impl Default for IdempotencyStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Axum middleware that enforces idempotency when an `idempotency-key` header
/// is present.
///
/// - If the key was seen before, the cached response is returned immediately.
/// - If the key is new, the request is processed normally and the response is
///   cached.
/// - If no key is present, the request passes through without caching.
pub async fn idempotency_middleware(
    req: Request,
    next: Next,
) -> Response {
    // TODO:
    // 1. Try to extract IdempotencyStore from request extensions.
    // 2. Check for the idempotency-key header.
    // 3. If key exists and is cached, return the cached response.
    // 4. Otherwise, call next.run(req), cache if key present, return response.
    let _ = &next;
    let _ = req;
    todo!("idempotency_middleware")
}
