# Building Block Prod-4: Deployment and Operations

**Prerequisites**: [Project: Performance & Profiling](../projects/prod-3/README.md).

Before starting [Project: Deployment Pipeline](../projects/prod-4/README.md), complete
the readings and exercises below.

## What to read

- [Dockerfile Best Practices for Rust](https://depot.dev/blog/rust-dockerfile-best-practices).
  Multi-stage builds, caching dependencies separately, choosing base images.

- [The Twelve-Factor App](https://12factor.net/).
  Methodology for building SaaS applications. Focus on config, logging, and
  disposability.

- [config crate documentation](https://docs.rs/config/latest/config/).
  Layered configuration from files, environment variables, and defaults.

- [Tokio graceful shutdown](https://tokio.rs/tokio/topics/shutdown).
  How to handle SIGTERM, drain connections, and shut down cleanly.

- [GitHub Actions for Rust](https://github.com/actions-rs).
  CI/CD setup for Rust projects.

## Key concepts

### Multi-Stage Docker Build

```dockerfile
# Stage 1: Build
FROM rust:1.77 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /usr/local/bin/
CMD ["server"]
```

### Configuration Layering

```
Defaults → config.toml → config.{env}.toml → Environment variables → CLI args
```

Each layer overrides the previous. Environment variables win.

### Graceful Shutdown

```rust
let (tx, rx) = tokio::sync::watch::channel(());
tokio::spawn(async move {
    tokio::signal::ctrl_c().await.unwrap();
    let _ = tx.send(());
});
axum::serve(listener, app)
    .with_graceful_shutdown(async move { rx.changed().await.ok(); })
    .await?;
```

## Exercises

**Exercise 1**: Write a multi-stage Dockerfile for a simple Rust binary.
Compare the image size vs a single-stage build.

**Exercise 2**: Use the `config` crate to load settings from both a TOML file
and environment variables. Verify that env vars override file values.

**Exercise 3**: Implement graceful shutdown: start a server, send SIGTERM,
verify that in-flight requests complete before the process exits.

## You're ready when...

- [ ] You can write an efficient multi-stage Dockerfile for Rust
- [ ] You understand the 12-factor app methodology
- [ ] You can implement graceful shutdown with tokio signals
- [ ] You can set up a basic CI pipeline for a Rust project

Next: [Project: Deployment Pipeline](../projects/prod-4/README.md)
