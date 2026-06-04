use std::time::Duration;

use taskforge_perf::cache::TaskCache;
use taskforge_perf::optimized::{
    calculate_percentile, collect_ids_optimized, collect_ids_slow, compress_response,
    decompress_response, process_tags_optimized, process_tags_slow, serialize_optimized,
    serialize_slow,
};
use taskforge_perf::Task;

// -----------------------------------------------------------------------
// Cache: basic operations
// -----------------------------------------------------------------------

#[tokio::test]
async fn cache_get_after_set_returns_value() {
    let cache = TaskCache::new(100, 60);
    cache.set("key1".into(), "value1".into()).await;
    let result = cache.get("key1").await;
    assert_eq!(result, Some("value1".to_string()));
}

#[tokio::test]
async fn cache_get_miss_returns_none() {
    let cache = TaskCache::new(100, 60);
    let result = cache.get("nonexistent").await;
    assert_eq!(result, None);
}

#[tokio::test]
async fn cache_ttl_expiry() {
    // Use a 1-second TTL so the test completes quickly.
    let cache = TaskCache::new(100, 1);
    cache.set("ephemeral".into(), "gone_soon".into()).await;

    // Should exist immediately.
    assert_eq!(cache.get("ephemeral").await, Some("gone_soon".to_string()));

    // Wait for the TTL to expire.
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Should be gone.
    assert_eq!(cache.get("ephemeral").await, None);
}

#[tokio::test]
async fn cache_invalidate_removes_entry() {
    let cache = TaskCache::new(100, 60);
    cache.set("key1".into(), "value1".into()).await;
    cache.invalidate("key1").await;
    assert_eq!(cache.get("key1").await, None);
}

#[tokio::test]
async fn cache_invalidate_prefix_removes_matching_entries() {
    let cache = TaskCache::new(100, 60);
    cache.set("tasks:list:page1".into(), "data1".into()).await;
    cache.set("tasks:list:page2".into(), "data2".into()).await;
    cache.set("tasks:detail:1".into(), "detail".into()).await;

    cache.invalidate_prefix("tasks:list:").await;

    assert_eq!(cache.get("tasks:list:page1").await, None);
    assert_eq!(cache.get("tasks:list:page2").await, None);
    // The detail entry should still be present.
    assert_eq!(cache.get("tasks:detail:1").await, Some("detail".to_string()));
}

#[tokio::test]
async fn cache_concurrent_access() {
    use std::sync::Arc;

    let cache = Arc::new(TaskCache::new(1000, 60));
    let mut handles = Vec::new();

    for i in 0..50 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(async move {
            let key = format!("concurrent:{i}");
            let value = format!("value:{i}");
            cache.set(key.clone(), value.clone()).await;
            let result = cache.get(&key).await;
            assert_eq!(result, Some(value));
        }));
    }

    for handle in handles {
        handle.await.expect("task panicked");
    }
}

// -----------------------------------------------------------------------
// Optimized functions: correctness (same results as slow versions)
// -----------------------------------------------------------------------

#[test]
fn process_tags_optimized_matches_slow() {
    let tags = vec![
        "Hello World".to_string(),
        "already-lowercase".to_string(),
        "UPPER CASE".to_string(),
        "MiXeD".to_string(),
    ];
    assert_eq!(process_tags_slow(&tags), process_tags_optimized(&tags));
}

#[test]
fn process_tags_optimized_empty_input() {
    let tags: Vec<String> = Vec::new();
    assert_eq!(process_tags_slow(&tags), process_tags_optimized(&tags));
}

#[test]
fn collect_ids_optimized_matches_slow() {
    let tasks: Vec<Task> = (0..10).map(|_| Task::sample()).collect();
    assert_eq!(collect_ids_slow(&tasks), collect_ids_optimized(&tasks));
}

#[test]
fn serialize_optimized_matches_slow() {
    let task = Task::sample();
    assert_eq!(serialize_slow(&task), serialize_optimized(&task));
}

// -----------------------------------------------------------------------
// Response compression
// -----------------------------------------------------------------------

#[test]
fn compress_decompress_roundtrip() {
    let payload = "x".repeat(2048); // large enough to benefit from compression
    let compressed = compress_response(payload.as_bytes());
    let decompressed = decompress_response(&compressed);
    assert_eq!(decompressed, payload.as_bytes());
}

#[test]
fn compress_small_payload_roundtrip() {
    let payload = b"tiny";
    let compressed = compress_response(payload);
    let decompressed = decompress_response(&compressed);
    assert_eq!(decompressed, payload);
}

// -----------------------------------------------------------------------
// Percentile calculation
// -----------------------------------------------------------------------

#[test]
fn percentile_p99_calculation() {
    let mut durations: Vec<Duration> = (1..=100).map(|ms| Duration::from_millis(ms)).collect();
    durations.sort();

    let p50 = calculate_percentile(&durations, 50.0).expect("p50");
    let p95 = calculate_percentile(&durations, 95.0).expect("p95");
    let p99 = calculate_percentile(&durations, 99.0).expect("p99");

    // p50 should be around 50ms, p95 around 95ms, p99 around 99ms.
    assert!(p50 >= Duration::from_millis(49) && p50 <= Duration::from_millis(51));
    assert!(p95 >= Duration::from_millis(94) && p95 <= Duration::from_millis(96));
    assert!(p99 >= Duration::from_millis(98) && p99 <= Duration::from_millis(100));
}

#[test]
fn percentile_empty_slice_returns_none() {
    let durations: Vec<Duration> = Vec::new();
    assert_eq!(calculate_percentile(&durations, 50.0), None);
}
