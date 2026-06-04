use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::Duration;

use crate::cache::Cache;
use crate::error::CacheError;

/// An in-memory cache (L1) backed by the moka async cache.
///
/// Supports TTL-based expiry, max capacity with LRU eviction, and
/// tag-based invalidation.
pub struct MemoryCache {
    /// The underlying moka async cache.
    _inner: moka::future::Cache<String, Vec<u8>>,
    /// Default TTL applied when no explicit TTL is provided.
    _default_ttl: Option<Duration>,
    /// Mapping from tags to the set of keys associated with that tag.
    _tags: Mutex<HashMap<String, HashSet<String>>>,
}

impl MemoryCache {
    /// Create a new in-memory cache.
    ///
    /// # Arguments
    /// - `max_capacity` — maximum number of entries before eviction.
    /// - `default_ttl` — default time-to-live for entries; `None` means no default expiry.
    pub fn new(max_capacity: u64, default_ttl: Option<Duration>) -> Self {
        // TODO: Build a moka::future::Cache with the given capacity and TTL.
        // TODO: Initialize the tag map.
        let _ = max_capacity;
        let _ = default_ttl;
        todo!()
    }

    /// Store a value and associate it with the given tags.
    ///
    /// This is an extended version of `set` that tracks tag-to-key mappings
    /// for later bulk invalidation.
    pub async fn set_with_tags(
        &self,
        _key: &str,
        _value: Vec<u8>,
        _ttl: Option<Duration>,
        _tags: &[&str],
    ) -> Result<(), CacheError> {
        // TODO: Insert into the moka cache.
        // TODO: Record the key under each tag in the tag map.
        todo!()
    }
}

#[async_trait]
impl Cache for MemoryCache {
    async fn get(&self, _key: &str) -> Option<Vec<u8>> {
        // TODO: Look up the key in the moka cache.
        todo!()
    }

    async fn set(
        &self,
        _key: &str,
        _value: Vec<u8>,
        _ttl: Option<Duration>,
    ) -> Result<(), CacheError> {
        // TODO: Insert into the moka cache with the given (or default) TTL.
        todo!()
    }

    async fn invalidate(&self, _key: &str) -> Result<(), CacheError> {
        // TODO: Remove the key from the moka cache.
        // TODO: Remove the key from all tag sets.
        todo!()
    }

    async fn invalidate_by_tag(&self, _tag: &str) -> Result<(), CacheError> {
        // TODO: Look up all keys for the tag.
        // TODO: Remove each key from the moka cache.
        // TODO: Clean up the tag mapping.
        todo!()
    }
}
