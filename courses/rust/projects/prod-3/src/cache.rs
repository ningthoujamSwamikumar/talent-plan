use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

/// A caching layer for task data, backed by moka's async cache.
///
/// Students implement all methods. The cache maps string keys to JSON string
/// values and supports TTL-based expiration plus explicit invalidation.
pub struct TaskCache {
    // TODO: store a moka::future::Cache<String, String> here
}

impl TaskCache {
    /// Create a new cache with the given maximum capacity and TTL in seconds.
    pub fn new(max_capacity: u64, ttl_seconds: u64) -> Self {
        todo!()
    }

    /// Retrieve a cached value by key, or None if absent / expired.
    pub async fn get(&self, key: &str) -> Option<String> {
        todo!()
    }

    /// Insert a key-value pair into the cache.
    pub async fn set(&self, key: String, value: String) {
        todo!()
    }

    /// Remove a single entry from the cache.
    pub async fn invalidate(&self, key: &str) {
        todo!()
    }

    /// Remove all entries whose key starts with `prefix`.
    ///
    /// This is useful for invalidating all list-related cache entries when the
    /// underlying data changes (e.g., prefix = "tasks:list:").
    pub async fn invalidate_prefix(&self, prefix: &str) {
        todo!()
    }
}
