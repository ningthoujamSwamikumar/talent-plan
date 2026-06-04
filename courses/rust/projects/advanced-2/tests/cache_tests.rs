use std::sync::Arc;
use std::time::Duration;

use resilience_middleware::cache::Cache;
use resilience_middleware::cache::memory::MemoryCache;
use resilience_middleware::cache::multi_tier::MultiTierCache;

// ---------------------------------------------------------------------------
// MemoryCache basic operations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn memory_cache_get_returns_none_on_miss() {
    let cache = MemoryCache::new(100, None);
    assert!(cache.get("nonexistent").await.is_none());
}

#[tokio::test]
async fn memory_cache_set_and_get_roundtrip() {
    let cache = MemoryCache::new(100, None);
    let value = b"hello".to_vec();
    cache.set("key1", value.clone(), None).await.unwrap();
    let retrieved = cache.get("key1").await;
    assert_eq!(retrieved, Some(value));
}

#[tokio::test]
async fn memory_cache_set_overwrites_existing_value() {
    let cache = MemoryCache::new(100, None);
    cache.set("key1", b"first".to_vec(), None).await.unwrap();
    cache.set("key1", b"second".to_vec(), None).await.unwrap();
    let retrieved = cache.get("key1").await;
    assert_eq!(retrieved, Some(b"second".to_vec()));
}

#[tokio::test]
async fn memory_cache_ttl_expiry() {
    let cache = MemoryCache::new(100, None);
    cache
        .set("ephemeral", b"gone soon".to_vec(), Some(Duration::from_millis(50)))
        .await
        .unwrap();

    // Should be present immediately.
    assert!(cache.get("ephemeral").await.is_some());

    // Wait for expiry.
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(cache.get("ephemeral").await.is_none());
}

#[tokio::test]
async fn memory_cache_invalidate_removes_entry() {
    let cache = MemoryCache::new(100, None);
    cache.set("key1", b"data".to_vec(), None).await.unwrap();
    cache.invalidate("key1").await.unwrap();
    assert!(cache.get("key1").await.is_none());
}

#[tokio::test]
async fn memory_cache_invalidate_nonexistent_key_is_ok() {
    let cache = MemoryCache::new(100, None);
    // Should not error even though the key does not exist.
    cache.invalidate("nope").await.unwrap();
}

// ---------------------------------------------------------------------------
// Tag-based invalidation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn memory_cache_tag_invalidation_removes_tagged_entries() {
    let cache = MemoryCache::new(100, None);
    cache
        .set_with_tags("proj:1:name", b"alpha".to_vec(), None, &["project:1"])
        .await
        .unwrap();
    cache
        .set_with_tags("proj:1:desc", b"bravo".to_vec(), None, &["project:1"])
        .await
        .unwrap();
    cache
        .set_with_tags("proj:2:name", b"charlie".to_vec(), None, &["project:2"])
        .await
        .unwrap();

    // Invalidate tag "project:1" — should remove the first two entries.
    cache.invalidate_by_tag("project:1").await.unwrap();

    assert!(cache.get("proj:1:name").await.is_none());
    assert!(cache.get("proj:1:desc").await.is_none());
    // "project:2" entries should be untouched.
    assert_eq!(cache.get("proj:2:name").await, Some(b"charlie".to_vec()));
}

#[tokio::test]
async fn memory_cache_invalidate_unknown_tag_is_ok() {
    let cache = MemoryCache::new(100, None);
    cache.invalidate_by_tag("nonexistent-tag").await.unwrap();
}

// ---------------------------------------------------------------------------
// Multi-tier cache
// ---------------------------------------------------------------------------

#[tokio::test]
async fn multi_tier_l1_hit_does_not_query_l2() {
    let l1 = Arc::new(MemoryCache::new(100, None));
    let l2 = Arc::new(MemoryCache::new(100, None));

    l1.set("key", b"from_l1".to_vec(), None).await.unwrap();
    l2.set("key", b"from_l2".to_vec(), None).await.unwrap();

    let multi = MultiTierCache::new(l1.clone(), l2.clone());
    // Should get the L1 value.
    assert_eq!(multi.get("key").await, Some(b"from_l1".to_vec()));
}

#[tokio::test]
async fn multi_tier_l1_miss_falls_through_to_l2_and_writes_through() {
    let l1 = Arc::new(MemoryCache::new(100, None));
    let l2 = Arc::new(MemoryCache::new(100, None));

    // Only L2 has the value.
    l2.set("key", b"from_l2".to_vec(), None).await.unwrap();

    let multi = MultiTierCache::new(l1.clone(), l2.clone());
    let result = multi.get("key").await;
    assert_eq!(result, Some(b"from_l2".to_vec()));

    // After the multi-tier get, L1 should now have the value (write-through).
    assert_eq!(l1.get("key").await, Some(b"from_l2".to_vec()));
}
