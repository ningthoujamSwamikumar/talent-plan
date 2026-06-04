# Building Block Prod-1: Observability — Logging, Tracing, and Metrics

**Prerequisites**: Phase 2 complete (Projects web-1 through web-4).

Before starting [Project: Observability Stack](../projects/prod-1/README.md), complete
the readings and exercises below.

## What to read

- [Tracing crate documentation](https://docs.rs/tracing/latest/tracing/).
  The standard instrumentation library for Rust. Understand spans, events, and
  subscribers.

- [Tracing subscriber documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/).
  How to configure output formatting, filtering, and layering.

- [Prometheus metric types](https://prometheus.io/docs/concepts/metric_types/).
  Four metric types: counter, gauge, histogram, summary. Know when to use each.

- [OpenTelemetry for Rust](https://opentelemetry.io/docs/languages/rust/).
  The vendor-neutral observability standard. Understand traces, spans, and how
  they propagate across service boundaries.

- [Google SRE Book — Monitoring Distributed Systems](https://sre.google/sre-book/monitoring-distributed-systems/).
  The "four golden signals": latency, traffic, errors, saturation.

## Key concepts

### Logging vs Tracing vs Metrics

| Concern | Tool | Purpose |
|---------|------|---------|
| Logging | `tracing` events | Record discrete events (errors, state changes) |
| Tracing | `tracing` spans | Track request flow across async boundaries |
| Metrics | `metrics` crate | Aggregate numerical measurements (p99, rate, count) |

### Structured Logging

```rust
tracing::info!(user_id = %user.id, action = "create_task", "Task created");
// Output: {"timestamp":"...","level":"INFO","user_id":"abc","action":"create_task","message":"Task created"}
```

### Spans and Context Propagation

```rust
#[tracing::instrument(skip(pool))]
async fn get_task(pool: &PgPool, id: Uuid) -> Result<Task> {
    // All events inside this function are tagged with the span
    tracing::debug!("Fetching task from database");
    // ...
}
```

### Prometheus Metrics

```rust
metrics::counter!("http_requests_total", "method" => "GET", "status" => "200").increment(1);
metrics::histogram!("http_request_duration_seconds").record(duration.as_secs_f64());
metrics::gauge!("active_connections").set(count as f64);
```

## Exercises

**Exercise 1**: Add `tracing` to a simple axum server. Use `#[tracing::instrument]`
on handler functions and observe the span output.

**Exercise 2**: Configure `tracing-subscriber` with JSON formatting and an
environment filter (`RUST_LOG=info,my_app=debug`).

**Exercise 3**: Add a counter for HTTP requests and a histogram for request
duration. Expose them on a `/metrics` endpoint.

## You're ready when...

- [ ] You can instrument functions with tracing spans
- [ ] You understand structured logging vs unstructured
- [ ] You know the four Prometheus metric types
- [ ] You can explain the "four golden signals" of monitoring

Next: [Project: Observability Stack](../projects/prod-1/README.md)
