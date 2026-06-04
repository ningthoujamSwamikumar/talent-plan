# Building Block Adv-2: Caching, Rate Limiting, and Resilience Patterns

**Prerequisites**: [Project: Message Queue Consumer](../projects/advanced-1/README.md).

Before starting [Project: Caching & Rate Limiting](../projects/advanced-2/README.md),
complete the readings and exercises below.

## What to read

- [Caching Patterns (AWS)](https://docs.aws.amazon.com/whitepapers/latest/database-caching-strategies-using-redis/caching-patterns.html).
  Cache-aside, read-through, write-through, write-behind. Know when each applies.

- [Rate Limiting Algorithms](https://blog.bytebytego.com/p/rate-limiting-fundamentals).
  Token bucket, leaky bucket, fixed window, sliding window. Token bucket is the
  most common for API rate limiting.

- [Circuit Breaker Pattern (Martin Fowler)](https://martinfowler.com/bliki/CircuitBreaker.html).
  Prevent cascading failures by failing fast when a downstream service is unhealthy.

- [Tower middleware documentation](https://docs.rs/tower/latest/tower/).
  How to write custom tower `Service` and `Layer` implementations.

- [Moka crate documentation](https://docs.rs/moka/latest/moka/).
  High-performance concurrent cache with TTL, max capacity, and eviction policies.

## Key concepts

### Multi-Tier Caching

```
Request → L1 (in-memory, ~1ms) → L2 (Redis, ~5ms) → Database (~50ms)
```

L1 handles hot data with sub-millisecond latency. L2 handles warm data shared
across instances. Database is the source of truth.

### Token Bucket Algorithm

```
Bucket capacity: 100 tokens
Refill rate: 10 tokens/second
Each request costs 1 token
If bucket is empty → reject (429 Too Many Requests)
```

### Circuit Breaker States

```
CLOSED → (failures exceed threshold) → OPEN
OPEN → (timeout expires) → HALF-OPEN
HALF-OPEN → (probe succeeds) → CLOSED
HALF-OPEN → (probe fails) → OPEN
```

## Exercises

**Exercise 1**: Implement token bucket rate limiting in ~50 lines. Track
remaining tokens and last refill time.

**Exercise 2**: Write a tower middleware that wraps a service and adds a
custom header to every response.

**Exercise 3**: Implement a circuit breaker that wraps a fallible function.
Track consecutive failures and open the circuit after 3.

## You're ready when...

- [ ] You can explain cache-aside vs write-through
- [ ] You can implement a token bucket rate limiter
- [ ] You understand circuit breaker state transitions
- [ ] You can write a tower middleware layer

Next: [Project: Caching & Rate Limiting](../projects/advanced-2/README.md)
