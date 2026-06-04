use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

use crate::cache::Cache;
use crate::error::CacheError;

/// A multi-tier cache that combines an L1 (fast, in-memory) cache with an
/// L2 (shared, e.g. Redis) cache.
///
/// **Read path:** L1 -> L2 -> miss. On an L2 hit the value is written
/// through to L1 before being returned.
///
/// **Write path:** writes go to both L1 and L2.
///
/// **Invalidation:** invalidates from both tiers.
pub struct MultiTierCache {
    /// The fast, in-process L1 cache.
    _l1: Arc<dyn Cache>,
    /// The shared L2 cache (e.g., Redis).
    _l2: Arc<dyn Cache>,
}

impl MultiTierCache {
    /// Create a new multi-tier cache.
    pub fn new(l1: Arc<dyn Cache>, l2: Arc<dyn Cache>) -> Self {
        // TODO: Store both cache tiers.
        let _ = l1;
        let _ = l2;
        todo!()
    }
}

#[async_trait]
impl Cache for MultiTierCache {
    async fn get(&self, _key: &str) -> Option<Vec<u8>> {
        // TODO: Check L1 first.
        // TODO: On L1 miss, check L2.
        // TODO: On L2 hit, write-through to L1, then return.
        // TODO: On both miss, return None.
        todo!()
    }

    async fn set(
        &self,
        _key: &str,
        _value: Vec<u8>,
        _ttl: Option<Duration>,
    ) -> Result<(), CacheError> {
        // TODO: Write to both L1 and L2.
        todo!()
    }

    async fn invalidate(&self, _key: &str) -> Result<(), CacheError> {
        // TODO: Invalidate from both L1 and L2.
        todo!()
    }

    async fn invalidate_by_tag(&self, _tag: &str) -> Result<(), CacheError> {
        // TODO: Invalidate by tag from both L1 and L2.
        todo!()
    }
}
