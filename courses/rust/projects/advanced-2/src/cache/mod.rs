pub mod memory;
pub mod multi_tier;
pub mod redis_cache;

use async_trait::async_trait;
use std::time::Duration;

use crate::error::CacheError;

/// A trait for asynchronous cache implementations.
///
/// Implementors must be safe to share across threads and async tasks.
#[async_trait]
pub trait Cache: Send + Sync {
    /// Retrieve a value by key. Returns `None` on cache miss.
    async fn get(&self, key: &str) -> Option<Vec<u8>>;

    /// Store a value with an optional TTL. If `ttl` is `None`, the entry
    /// does not expire (though it may still be evicted by capacity limits).
    async fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        ttl: Option<Duration>,
    ) -> Result<(), CacheError>;

    /// Remove a single entry by key.
    async fn invalidate(&self, key: &str) -> Result<(), CacheError>;

    /// Remove all entries associated with the given tag.
    async fn invalidate_by_tag(&self, tag: &str) -> Result<(), CacheError>;
}
