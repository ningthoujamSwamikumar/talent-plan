use std::time::Duration;

use resilience_middleware::rate_limiter::token_bucket::TokenBucket;

#[test]
fn token_bucket_starts_full() {
    let bucket = TokenBucket::new(10, 1.0);
    assert_eq!(bucket.remaining(), 10);
}

#[test]
fn token_bucket_acquire_decrements_tokens() {
    let bucket = TokenBucket::new(10, 1.0);
    assert!(bucket.try_acquire());
    assert_eq!(bucket.remaining(), 9);
}

#[test]
fn token_bucket_acquire_n_decrements_by_n() {
    let bucket = TokenBucket::new(10, 1.0);
    assert!(bucket.try_acquire_n(5));
    assert_eq!(bucket.remaining(), 5);
}

#[test]
fn token_bucket_rejects_when_exhausted() {
    let bucket = TokenBucket::new(2, 0.0); // no refill
    assert!(bucket.try_acquire());
    assert!(bucket.try_acquire());
    // Bucket is now empty.
    assert!(!bucket.try_acquire());
}

#[test]
fn token_bucket_acquire_n_rejects_when_insufficient() {
    let bucket = TokenBucket::new(3, 0.0);
    // Asking for more than available should fail and not consume tokens.
    assert!(!bucket.try_acquire_n(4));
    assert_eq!(bucket.remaining(), 3);
}

#[tokio::test]
async fn token_bucket_refills_over_time() {
    let bucket = TokenBucket::new(5, 100.0); // 100 tokens/sec
    // Drain it.
    for _ in 0..5 {
        bucket.try_acquire();
    }
    assert_eq!(bucket.remaining(), 0);

    // Wait long enough for at least 1 token to refill.
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(bucket.remaining() >= 1);
}

#[test]
fn token_bucket_does_not_exceed_capacity() {
    // Even if time passes, remaining should never exceed capacity.
    let bucket = TokenBucket::new(5, 1000.0);
    // remaining is already full at creation.
    assert_eq!(bucket.remaining(), 5);
    // Another call should still show <= capacity.
    assert!(bucket.remaining() <= 5);
}

#[test]
fn token_bucket_concurrent_access_is_safe() {
    use std::sync::Arc;
    use std::thread;

    let bucket = Arc::new(TokenBucket::new(1000, 0.0));
    let mut handles = vec![];

    for _ in 0..10 {
        let b = Arc::clone(&bucket);
        handles.push(thread::spawn(move || {
            let mut acquired = 0u32;
            for _ in 0..100 {
                if b.try_acquire() {
                    acquired += 1;
                }
            }
            acquired
        }));
    }

    let total: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    // With 1000 tokens and no refill, exactly 1000 should have been granted.
    assert_eq!(total, 1000);
}
