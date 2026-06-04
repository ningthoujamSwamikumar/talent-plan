# Building Block: Failure Scenarios and Chaos Engineering

Production systems fail. Disks fill up, networks partition, databases go
unresponsive, memory runs out, certificates expire. Senior engineers don't just
handle the happy path — they design for failure and test that their failure
handling works.

## Readings

- [Release It! (Michael Nygard)](https://pragprog.com/titles/mnee2/release-it-second-edition/)
  — the definitive book on production failure patterns. Read chapters on
  stability patterns (circuit breakers, bulkheads, timeouts).
- [The Netflix Simian Army](https://netflixtechblog.com/the-netflix-simian-army-16e57fbab116)
  — how Netflix tests failure resilience in production.
- [Jepsen: analyses of distributed systems](https://jepsen.io/analyses) — read
  2-3 analyses to see how real distributed systems fail. Note the methodology.
- [PostgreSQL failure modes](https://www.postgresql.org/docs/current/monitoring-stats.html)
  — what happens when connections are exhausted, disk is full, or WAL grows too
  large.
- [Linux OOM killer explained](https://www.kernel.org/doc/gorman/html/understand/understand016.html)
  — what happens when your Rust service runs out of memory on Linux.

## Key Concepts

**The five categories of production failure:**

1. **Resource exhaustion**: Disk full, memory OOM, file descriptors exhausted,
   connection pool drained, thread pool saturated
2. **Network failures**: Partition, latency spike, packet loss, DNS failure,
   TLS certificate expiry
3. **Dependency failures**: Database down, external API unresponsive, message
   queue full, cache eviction storm
4. **Logic errors**: Race condition, deadlock, infinite loop, data corruption,
   inconsistent state after partial failure
5. **Operational errors**: Bad deploy, misconfiguration, missing environment
   variable, wrong permissions

**Stability patterns (from Release It!):**

| Pattern | What It Does | When To Use |
|---------|-------------|-------------|
| **Timeout** | Limit how long you wait for a response | Every external call |
| **Circuit Breaker** | Stop calling a failing service | Dependency failures |
| **Bulkhead** | Isolate failures to one subsystem | Multi-tenant systems |
| **Retry with backoff** | Retry transient failures | Flaky dependencies |
| **Fallback** | Use a degraded response when primary fails | Non-critical features |
| **Shed load** | Reject requests when overloaded | Traffic spikes |
| **Health checks** | Detect and remove unhealthy instances | All services |

**Designing for failure:**

1. **Every external call needs a timeout.** No exceptions. An HTTP call without
   a timeout will block a thread/task forever when the remote hangs.

2. **Every retry needs a limit and backoff.** Unlimited retries with no delay
   create a retry storm that makes outages worse.

3. **Every resource needs a limit.** Connection pools, thread pools, channel
   buffers, cache sizes — all bounded. Unbounded = OOM eventually.

4. **Every failure needs observability.** If a circuit breaker trips, a timeout
   fires, or a retry exhausts — log it with structured data (which service,
   which endpoint, how long, what error).

## Exercises

1. **Map your failure modes.** Take your capstone project and list every
   external dependency (database, Redis, external APIs). For each, answer:
   What happens if it's down? What happens if it's slow (10x normal latency)?
   What happens if it returns garbage data? Do you have timeouts everywhere?

2. **Test timeout behavior.** Write a test that starts a TCP listener that
   accepts connections but never responds. Connect to it from your service
   with and without a timeout. Observe the difference.

3. **Test resource exhaustion.** Write a test that opens connections to your
   service faster than it can process them. What happens when the connection
   pool is full? Does the service crash, return 503, or hang?

4. **Design a graceful degradation strategy.** Your service depends on a
   recommendation engine that's flaky. Design a fallback: when the engine is
   down, return cached/default recommendations instead of erroring. Implement
   the circuit breaker + fallback pattern.

---

## Checklist

- [ ] You can identify the failure modes of your system's dependencies
- [ ] You set timeouts on every external call
- [ ] You use circuit breakers for flaky dependencies
- [ ] You test what happens when dependencies fail (not just when they work)
- [ ] You understand bulkhead isolation and load shedding
- [ ] You design fallbacks for non-critical features

Next: [Project: Chaos Engineering & Failure Recovery](../projects/prod-6/README.md)
