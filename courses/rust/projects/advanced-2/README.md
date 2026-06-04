# Project: Caching and Rate Limiting

**Phase 4, Project 2 — Resilience Middleware**

In this project you will build a middleware library that provides multi-tier
caching, rate limiting, and circuit breaking. These are foundational patterns
for building resilient, high-performance networked services.

By the end of this project you will understand how production systems layer
fast in-process caches in front of shared caches, how token-bucket rate
limiters protect services from overload, and how circuit breakers prevent
cascading failures.

## Part 1: Cache Trait

Define the `Cache` trait in `src/cache/mod.rs`. The trait must be
object-safe and async, so we use the `async_trait` crate.

```rust
#[async_trait]
pub trait Cache: Send + Sync {
    async fn get(&self, key: &str) -> Option<Vec<u8>>;
    async fn set(&self, key: &str, value: Vec<u8>, ttl: Option<Duration>) -> Result<(), CacheError>;
    async fn invalidate(&self, key: &str) -> Result<(), CacheError>;
    async fn invalidate_by_tag(&self, tag: &str) -> Result<(), CacheError>;
}
```

**Goals:**

- Understand why `Send + Sync` bounds are required for shared async caches.
- Think about how tag-based invalidation differs from key-based invalidation.

## Part 2: In-Memory Cache (L1)

Implement `MemoryCache` in `src/cache/memory.rs` using the **moka** crate's
async cache. Your implementation must support:

- **TTL (time-to-live):** entries expire after a configurable duration.
- **Max capacity:** the cache evicts least-recently-used entries when full.
- **Tag tracking:** maintain a mapping from tags to sets of keys so that
  `invalidate_by_tag` can remove all related entries in one call.

**Hints:**

- `moka::future::Cache` handles TTL and capacity for you.
- Store the tag-to-key mapping in a `DashMap<String, HashSet<String>>` or
  a `Mutex<HashMap<...>>`.

## Part 3: Redis Cache (L2)

Implement `RedisCache` in `src/cache/redis_cache.rs` using the **redis**
crate with `tokio-comp` for async support.

- `get` / `set` map directly to Redis `GET` / `SET` (with `EX` for TTL).
- Tag tracking can use Redis Sets: for each tag, maintain a set of keys
  that belong to it.
- `invalidate_by_tag` fetches the set members and deletes them all.

**Hints:**

- Use `redis::aio::MultiplexedConnection` for concurrent access.
- Use `SADD` / `SMEMBERS` / `DEL` for tag management.

## Part 4: Multi-Tier Cache

Implement `MultiTierCache` in `src/cache/multi_tier.rs`. It combines an L1
(in-memory) and L2 (Redis) cache:

1. **Read path:** check L1 first. On miss, check L2. On L2 hit,
   **write-through** the value back into L1 before returning it.
2. **Write path:** write to both L1 and L2.
3. **Invalidation:** invalidate from both tiers.

**Goals:**

- Understand the trade-offs of write-through vs. write-behind caching.
- Think about what happens when L2 is unavailable — should L1 still work?

## Part 5: Tag-Based Invalidation

Tag-based invalidation lets you associate cache entries with logical groups.
For example, every cache entry related to "project 123" might be tagged
`"project:123"`. When project 123 is updated, a single call to
`invalidate_by_tag("project:123")` removes all stale entries.

**Requirements:**

- When calling `set`, the caller can associate zero or more tags with the
  entry (extend the trait or add a `set_with_tags` method).
- `invalidate_by_tag` must remove the entry from every tier and clean up
  the tag-to-key mappings.

## Part 6: Token Bucket Rate Limiter

Implement a **token bucket** rate limiter in
`src/rate_limiter/token_bucket.rs`.

The token bucket algorithm works as follows:

1. A bucket has a maximum **capacity** of tokens.
2. Tokens are added at a constant **refill rate** (tokens per second).
3. Each request consumes one token. If no tokens are available, the
   request is rejected.
4. The bucket never holds more tokens than its capacity.

**Public API:**

```rust
impl TokenBucket {
    pub fn new(capacity: u32, refill_rate: f64) -> Self;
    pub fn try_acquire(&self) -> bool;      // consume 1 token
    pub fn try_acquire_n(&self, n: u32) -> bool;  // consume n tokens
    pub fn remaining(&self) -> u32;         // tokens currently available
}
```

**Hints:**

- Use lazy refilling: compute how many tokens have accumulated since the
  last access instead of running a background timer.
- Protect mutable state with `Mutex` or atomics.

## Part 7: Distributed Rate Limiting

Extend rate limiting to work across multiple service instances by storing
token state in Redis.

- Use a Lua script or `MULTI`/`EXEC` to atomically check-and-decrement
  the token count.
- Handle Redis unavailability gracefully (fail open or fail closed,
  depending on your policy).

## Part 8: Circuit Breaker

Implement a circuit breaker in `src/circuit_breaker.rs`.

A circuit breaker monitors calls to an external service and prevents
cascading failures:

| State      | Behavior                                        |
|------------|-------------------------------------------------|
| **Closed** | Requests pass through. Failures are counted.    |
| **Open**   | Requests are immediately rejected.              |
| **HalfOpen** | One probe request is allowed through.        |

**Transitions:**

- Closed -> Open: when failure count reaches the threshold.
- Open -> HalfOpen: after a configurable timeout elapses.
- HalfOpen -> Closed: when the probe request succeeds.
- HalfOpen -> Open: when the probe request fails.

**Public API:**

```rust
impl CircuitBreaker {
    pub fn new(failure_threshold: u32, timeout: Duration) -> Self;
    pub fn state(&self) -> CircuitState;
    pub async fn call<F, T, E>(&self, f: F) -> Result<T, CircuitBreakerError<E>>;
    pub fn record_success(&self);
    pub fn record_failure(&self);
}
```

---

## Testing

Run all tests with:

```
cargo test
```

The test suite covers:

- **Cache tests** (~10 tests): get/set round-trip, TTL expiry,
  key invalidation, tag-based invalidation, multi-tier fallback
  (L1 miss falls through to L2, write-through on miss).
- **Rate limiter tests** (~8 tests): allows requests within limit,
  rejects when exhausted, refills tokens over time, concurrent access
  safety, burst handling.
- **Circuit breaker tests** (~8 tests): starts in closed state, opens
  after threshold failures, rejects calls in open state, transitions to
  half-open after timeout, closes on successful probe, reopens on failed
  probe.
