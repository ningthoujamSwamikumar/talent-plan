# Project: Deployment Pipeline

**Phase 3, Project 4 — taskforge-deploy**

In this project you will containerize, configure, and add graceful shutdown to the
TaskForge service. By the end you will have a production-grade deployment story:
layered configuration, feature flags, clean shutdown, a multi-stage Docker build,
health probes, and a CI pipeline.

---

## Part 1: Configuration Management

Use the `config` crate to build a layered configuration system. Values are resolved
in the following order (last wins):

1. **Compiled defaults** — sensible values baked into `config/default.toml`.
2. **Environment-specific file** — e.g. `config/production.toml` selected by the
   `APP_ENV` environment variable.
3. **Environment variables** — prefixed with `APP_` and separated by `__`
   (double underscore) for nested keys. For example `APP_SERVER__PORT=8080`.
4. **CLI overrides** — optional; students may add `clap` later.

Define an `AppConfig` struct with the following nested sections:

| Section       | Fields                                                        |
|---------------|---------------------------------------------------------------|
| `server`      | `host`, `port`, `shutdown_timeout_secs`                       |
| `database`    | `url`, `max_connections`                                      |
| `features`    | `enable_search`, `enable_websockets`, `max_tasks_per_project` |

### Exercises

1. Implement `AppConfig::load()` so that it reads `config/default.toml`, then
   optionally overlays a file chosen by `APP_ENV`, then overlays environment
   variables prefixed with `APP_`.
2. Write tests that verify defaults load correctly and that environment variables
   override file values.
3. Add validation: `port` must be > 0, `max_connections` must be >= 1,
   `max_tasks_per_project` must be >= 1.

---

## Part 2: Feature Flags

Implement a simple in-memory feature flag system backed by the `FeatureFlags`
section of configuration.

### Requirements

- `FeatureFlagService` wraps the flags in an `Arc` so handlers can read them
  cheaply.
- Provide a `is_enabled(&self, flag: &str) -> bool` method that matches on
  known flag names.
- Handlers should check the flag before executing gated logic and return
  `404 Not Found` (or a clear JSON error) when the feature is disabled.

### Exercises

1. Implement `FeatureFlagService` with `is_enabled`.
2. Write a handler guard or middleware that checks a named flag and short-circuits
   if the feature is off.
3. Write tests toggling flags on and off and asserting the correct HTTP status.

---

## Part 3: Graceful Shutdown

When the process receives `SIGTERM` (or `ctrl-c` during development) it must:

1. **Stop accepting new connections** immediately.
2. **Drain in-flight requests** — give them up to `shutdown_timeout_secs` to
   finish.
3. **Flush** any buffered logs or metrics.
4. **Exit** with code 0.

### Design

- Use `tokio::signal::ctrl_c()` and `tokio::signal::unix::signal(SignalKind::terminate())`
  to listen for shutdown signals.
- Create a `shutdown_signal()` async function that resolves when either signal
  fires.
- Pass it to `axum::serve(...).with_graceful_shutdown(shutdown_signal())`.
- Wrap the server future with `tokio::time::timeout` using the configured
  deadline.

### Exercises

1. Implement `shutdown_signal()`.
2. Implement `graceful_shutdown()` that coordinates draining and flushing.
3. Test that in-flight requests complete before the server exits.
4. Test that the server force-exits after the timeout deadline.

---

## Part 4: Dockerfile

Write a multi-stage Dockerfile using `cargo-chef` to cache dependency builds.

### Stages

| Stage       | Base Image              | Purpose                                    |
|-------------|-------------------------|--------------------------------------------|
| `chef`      | `rust:1.77`             | Install `cargo-chef`                       |
| `planner`   | `chef`                  | `cargo chef prepare` to create recipe.json |
| `builder`   | `chef`                  | `cargo chef cook` + `cargo build --release`|
| `runtime`   | `debian:bookworm-slim`  | Copy binary, install ca-certificates       |

### Exercises

1. Build the image locally: `docker build -t taskforge-deploy .`
2. Run it: `docker run -p 3000:3000 taskforge-deploy`
3. Verify the health endpoint responds.
4. Measure the final image size — aim for under 100 MB.

---

## Part 5: Health Checks for Container Orchestration

Expose two health endpoints:

| Endpoint           | Purpose             | Logic                                         |
|--------------------|---------------------|-----------------------------------------------|
| `GET /healthz`     | **Liveness** probe  | Returns `200` if the process is alive.        |
| `GET /readyz`      | **Readiness** probe | Returns `200` only when the service is ready to accept traffic (e.g. config loaded, dependencies reachable). Returns `503` during startup and during shutdown draining. |

### State Machine

```
Starting  ──>  Ready  ──>  Draining  ──>  Stopped
```

Use an `Arc<AtomicU8>` (or an enum behind a `RwLock`) shared between the
health handlers and the shutdown coordinator.

### Exercises

1. Implement `AppState` with a `readiness` field.
2. Wire `/healthz` and `/readyz` handlers.
3. Transition to `Draining` when a shutdown signal arrives.
4. Write tests asserting `/readyz` returns `503` before the service is ready and
   after shutdown begins.

---

## Part 6: CI Pipeline

Create a GitHub Actions workflow (`.github/workflows/ci.yml`) that runs on every
push and pull request to `main`. The pipeline should have the following jobs:

| Job         | Steps                                                         |
|-------------|---------------------------------------------------------------|
| `lint`      | `cargo clippy -- -D warnings`                                |
| `format`    | `cargo fmt --check`                                          |
| `test`      | `cargo test`                                                 |
| `build`     | `cargo build --release`                                      |
| `container` | `docker build -t taskforge-deploy .`                         |

### Exercises

1. Read through the workflow and understand each step.
2. Add caching for the Cargo registry and target directory using
   `actions/cache`.
3. (Stretch) Add a step that pushes the container image to GitHub Container
   Registry (ghcr.io) on merges to `main`.

---

## Getting Started

```bash
# Load default config and start the server
cargo run --bin server

# Override port via environment variable
APP_SERVER__PORT=9090 cargo run --bin server

# Run tests
cargo test

# Build Docker image
docker build -t taskforge-deploy .
docker run -p 3000:3000 taskforge-deploy
```

Good luck!
