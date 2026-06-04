# Project: Chaos Engineering & Failure Recovery

**Phase 3 — Production Engineering**

Production systems fail. In this project you will intentionally break your
service in every way that matters and verify it recovers correctly. You will
simulate disk failures, network partitions, database outages, memory pressure,
and cascading failures — then build the stability patterns that handle them.

## Prerequisites

- Completion of prod-1 through prod-5
- Building block bb-chaos (Failure Scenarios and Chaos Engineering)
- Docker (for simulating infrastructure failures)

## Deliverables

- [ ] A service with circuit breakers, timeouts, retries, and bulkheads
- [ ] Tests for 6 failure scenarios
- [ ] Graceful degradation under dependency failure
- [ ] Load shedding under overload
- [ ] A failure mode analysis document for your service

---

## Part 1: Timeout Everything

Take an axum service with 3 external dependencies (database, cache, external
API — all simulated). Add timeouts to every external call.

**Requirements:**
1. Database queries: 5-second timeout
2. Cache operations: 500ms timeout
3. External API calls: 3-second timeout
4. Overall request timeout: 10 seconds (tower `Timeout` layer)

**Implementation:**
```rust
// Database with timeout
let result = tokio::time::timeout(
    Duration::from_secs(5),
    sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = $1", id)
        .fetch_optional(&pool)
).await??;

// External API with timeout (reqwest has built-in timeout)
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(3))
    .connect_timeout(Duration::from_secs(1))
    .build()?;
```

**Simulate failure:** Create a mock external service that randomly delays
responses by 0-30 seconds. Verify your service returns an error within the
timeout, not after 30 seconds.

**Tests to pass:**
- `timeout_prevents_hang` — request completes within 11 seconds even when
  dependency hangs forever
- `timeout_error_is_observable` — timeout produces a structured log with
  dependency name, timeout duration, and trace ID

## Part 2: Circuit Breaker

Implement a circuit breaker for the external API dependency.

**States:**
- **Closed** (normal): requests pass through
- **Open** (tripped): requests fail immediately without calling the dependency
- **Half-Open** (testing): one request is allowed through to test recovery

**Configuration:**
- Trip after 5 consecutive failures
- Stay open for 30 seconds
- Half-open allows 1 probe request

**Implementation:** Build this as a `tower::Layer` so it can wrap any service.

```rust
pub struct CircuitBreakerLayer {
    failure_threshold: u32,
    recovery_timeout: Duration,
}

// State machine:
enum State {
    Closed { consecutive_failures: u32 },
    Open { opened_at: Instant },
    HalfOpen,
}
```

**Tests to pass:**
- `circuit_breaker_trips_after_failures` — 5 failures trips the breaker
- `circuit_breaker_fails_fast` — open breaker returns error in <1ms
- `circuit_breaker_recovers` — after recovery timeout, allows probe request
- `circuit_breaker_resets_on_success` — successful probe closes the breaker

## Part 3: Retry with Backoff

Implement retry middleware for transient failures.

**Requirements:**
- Retry on: connection refused, timeout, 503 Service Unavailable
- Do NOT retry on: 400, 401, 403, 404 (client errors are not transient)
- Maximum 3 retries
- Exponential backoff: 100ms, 200ms, 400ms (with +-20% jitter)
- Total retry budget: if >25% of requests in the last minute are retries,
  stop retrying (prevent retry storm)

**Jitter implementation:**
```rust
fn backoff_with_jitter(attempt: u32, base: Duration) -> Duration {
    let backoff = base * 2u32.pow(attempt);
    let jitter = backoff.mul_f64(rand::thread_rng().gen_range(0.8..1.2));
    jitter
}
```

**Tests to pass:**
- `retry_succeeds_on_transient_failure` — fails twice, succeeds on third
- `retry_does_not_retry_client_errors` — 400 is not retried
- `retry_respects_budget` — under heavy failure, retries are throttled
- `retry_uses_backoff` — second retry waits longer than first

## Part 4: Bulkhead Isolation

Implement bulkhead isolation so that a slow dependency doesn't consume all
resources.

**Requirements:**
- Separate semaphores for each dependency:
  - Database: max 20 concurrent requests
  - Cache: max 50 concurrent requests  
  - External API: max 10 concurrent requests
- When a bulkhead is full, return 503 immediately (don't queue)

**Implementation:**
```rust
pub struct Bulkhead {
    semaphore: Arc<Semaphore>,
    name: String,
}

impl Bulkhead {
    pub async fn execute<F, T>(&self, f: F) -> Result<T, BulkheadError>
    where
        F: Future<Output = T>,
    {
        let _permit = self.semaphore.try_acquire()
            .map_err(|_| BulkheadError::Full(self.name.clone()))?;
        Ok(f.await)
    }
}
```

**Tests to pass:**
- `bulkhead_limits_concurrent` — exceeding limit returns 503
- `bulkhead_isolates_dependencies` — slow database doesn't affect cache calls
- `bulkhead_releases_on_completion` — permits are released after request

## Part 5: Load Shedding

When the service is overloaded, shed load gracefully instead of slowing down
for everyone.

**Requirements:**
1. Track current in-flight requests with an `AtomicU64` counter
2. When in-flight exceeds threshold (e.g., 100), reject new requests with 503
3. Add a `Retry-After` header to 503 responses
4. High-priority requests (identified by header) bypass the limit
5. Log every shed request for monitoring

**Implementation:** Build as axum middleware.

**Tests to pass:**
- `load_shedding_returns_503` — excess requests get 503
- `load_shedding_includes_retry_after` — 503 has Retry-After header
- `load_shedding_allows_priority` — priority requests pass through
- `load_shedding_recovers` — after load drops, new requests succeed

## Part 6: Graceful Degradation

When the cache (Redis) is down, the service should degrade gracefully instead
of erroring.

**Requirements:**
1. Cache miss → query database (normal path)
2. Cache unavailable → query database directly (degraded path)
3. Log degradation with `tracing::warn!`
4. Expose degradation in health check (`/health` returns `degraded` status)
5. When cache recovers, service automatically returns to normal

**Implementation pattern:**
```rust
async fn get_task(id: Uuid, cache: &Cache, db: &PgPool) -> Result<Task> {
    match cache.get(id).await {
        Ok(Some(task)) => Ok(task),
        Ok(None) => {
            let task = db_get_task(db, id).await?;
            let _ = cache.set(id, &task).await; // ignore cache write failure
            Ok(task)
        }
        Err(_) => {
            tracing::warn!("cache unavailable, falling back to database");
            db_get_task(db, id).await
        }
    }
}
```

**Tests to pass:**
- `degradation_serves_without_cache` — service works when cache is down
- `degradation_health_reports_status` — health endpoint shows degraded state
- `degradation_recovers_automatically` — service returns to normal when cache
  comes back

## Part 7: Failure Mode Analysis

Write `FAILURE_MODES.md` for the complete service:

| Dependency | Failure Mode | Detection | Response | Recovery |
|-----------|-------------|-----------|----------|----------|
| Database | Down | Connection timeout | Return 503 | Circuit breaker, retry |
| Database | Slow | Query timeout | Return 503 after 5s | Timeout, shed load |
| Cache | Down | Connection error | Degrade to DB-only | Auto-recovery |
| Cache | Slow | Timeout | Skip cache, query DB | Timeout |
| External API | Down | Consecutive failures | Circuit breaker open | Probe after 30s |
| External API | Slow | Request timeout | Fail after 3s | Retry with backoff |
| Service | Overloaded | In-flight counter | Shed load (503) | Counter decreases |
| Service | Memory | OOM signal | Graceful shutdown | Restart |

This document is a real deliverable you'd produce as a senior engineer.

## Testing

```
cargo test
```

## What You Will Learn

- How to implement timeouts for every external dependency
- How to build circuit breakers as tower middleware
- Retry strategies with exponential backoff and jitter
- Bulkhead isolation to prevent cascading failures
- Load shedding to protect service stability under overload
- Graceful degradation when dependencies fail
- How to analyze and document failure modes systematically
