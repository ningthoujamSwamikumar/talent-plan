# Project: Observability Stack

In this project you will integrate tracing, metrics, and health checks into a
production web service built with Axum. By the end you will have a service that
emits structured JSON logs, records Prometheus metrics, propagates request IDs
end-to-end, and exposes health endpoints suitable for container orchestrators.

## Prerequisites

- Completion of the networking projects (or equivalent Axum experience).
- Familiarity with the `tower` middleware model.
- A working Rust toolchain (edition 2021).

---

## Part 1: Structured Logging

**Goal:** configure `tracing-subscriber` so that every log line is machine-readable
JSON with an adjustable verbosity level.

1. In `src/lib.rs`, create a public function `init_tracing()` that:
   - Builds a `tracing_subscriber::fmt` subscriber.
   - Uses `tracing_subscriber::EnvFilter` initialised from the `RUST_LOG`
     environment variable (defaulting to `info`).
   - Formats output as JSON (`fmt::format::Json`).
   - Installs the subscriber as the global default.
2. Verify that setting `RUST_LOG=debug` increases the verbosity of your output.
3. Add a few `tracing::info!` and `tracing::debug!` calls in a test handler and
   confirm the JSON output contains the fields `level`, `target`, `timestamp`,
   and your custom message.

**Hints:**
- `tracing_subscriber::fmt().json().with_env_filter(...)` is the builder chain
  you need.
- The `tracing::instrument` attribute macro automatically creates a span for a
  function.

---

## Part 2: Request Tracing

**Goal:** every inbound HTTP request should live inside its own tracing span so
that all log lines emitted while handling that request are correlated.

1. In `src/middleware.rs`, create a function `trace_layer()` that returns a
   configured `tower_http::trace::TraceLayer`.
   - The span should include the HTTP method, URI path, and a unique request ID.
   - On response, log the status code and elapsed time at `INFO` level.
2. Wire the layer into the Axum router in `src/bin/server.rs`.
3. Write a test that sends a request and asserts the response carries the
   expected tracing span attributes (method, path).

**Hints:**
- `TraceLayer::new_for_http()` gives you sensible defaults; customise with
  `.make_span_with(...)` and `.on_response(...)`.
- Use `tracing::instrument` on individual handler functions to add
  handler-level spans nested inside the request span.

---

## Part 3: Prometheus Metrics

**Goal:** expose a `/metrics` endpoint that returns counters, histograms, and
gauges in Prometheus exposition format.

1. In `src/metrics_setup.rs`, write `install_prometheus_recorder()` that:
   - Creates a `PrometheusBuilder`, installs it as the global recorder, and
     returns the `PrometheusHandle` (needed to render the text output).
2. Define the following metrics (use the `metrics` crate macros):
   - **Counter** `http_requests_total` -- incremented on every request, labeled
     by method and path.
   - **Histogram** `http_request_duration_seconds` -- records the elapsed time
     of every request.
   - **Gauge** `http_active_connections` -- incremented when a request starts,
     decremented when it completes.
3. Create a handler `GET /metrics` that calls `handle.render()` and returns the
   text body with content-type `text/plain`.
4. Write tests that:
   - Hit `/metrics` and assert the response is valid Prometheus text.
   - Confirm the counter and histogram appear in the output after sending a
     request to another endpoint.

**Hints:**
- `metrics::counter!("http_requests_total", "method" => method).increment(1);`
- The histogram can be recorded with `metrics::histogram!(...).record(elapsed)`.

---

## Part 4: Health Endpoints

**Goal:** add Kubernetes-style health probes.

1. In `src/health.rs`, implement two handlers:
   - `GET /health/live` -- always returns `200 OK` with a JSON body
     `{"status": "alive"}`. This tells the orchestrator the process is running.
   - `GET /health/ready` -- returns `200 OK` with `{"status": "ready"}` when
     all dependencies are reachable; returns `503 Service Unavailable` with
     `{"status": "not_ready", "reason": "..."}` otherwise.
2. For readiness, accept an `AppState` that contains a flag or a function the
   handler calls to check dependency health (e.g., a database ping simulation).
3. Write tests:
   - `/health/live` always returns 200.
   - `/health/ready` returns 200 when the dependency check passes.
   - `/health/ready` returns 503 when the dependency check fails.

---

## Part 5: Custom Tracing Layer

**Goal:** write your own `tower::Layer` / `tower::Service` that records per-request
latency into the `http_request_duration_seconds` histogram.

1. In `src/middleware.rs`, define:
   - `MetricsLayer` -- implements `tower::Layer`.
   - `MetricsService<S>` -- wraps an inner service `S`, implements
     `tower::Service<Request<Body>>`.
2. In the `Service::call` implementation:
   - Record the start time.
   - Call the inner service.
   - On completion, compute elapsed duration and record it to the histogram.
   - Increment the request counter and manage the active-connections gauge.
3. Wire `MetricsLayer` into the middleware stack in `src/bin/server.rs`.

**Hints:**
- Use `std::time::Instant` for timing.
- Return a `Pin<Box<dyn Future<...>>>` (or use an async block) from `call`.

---

## Part 6: Request ID Propagation

**Goal:** every response should carry an `X-Request-ID` header. If the client
sends one, echo it back; otherwise generate a new UUID v4.

1. Use `tower_http::request_id::SetRequestIdLayer` and
   `tower_http::request_id::PropagateRequestIdLayer` (or implement your own
   layer) so that:
   - A missing `X-Request-ID` is populated with a new `Uuid::new_v4()`.
   - The ID is inserted into the current tracing span as a field.
   - The ID appears in the response headers.
2. Write tests:
   - A request without `X-Request-ID` receives a valid UUID in the response.
   - A request with a supplied `X-Request-ID` gets the same value echoed back.

---

## Summary

After completing all six parts your service will:

| Capability | Endpoint / Mechanism |
|---|---|
| Structured JSON logs | `tracing-subscriber` with JSON formatter |
| Per-request spans | `TraceLayer` + `#[instrument]` |
| Prometheus metrics | `GET /metrics` |
| Liveness probe | `GET /health/live` |
| Readiness probe | `GET /health/ready` |
| Request ID propagation | `X-Request-ID` header on every response |

Run the full test suite with:

```bash
cargo test
```
