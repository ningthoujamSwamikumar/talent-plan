# Capstone: Production Backend Platform

Phase 7 -- the final integrative project of the Practical Networked Applications in Rust course.

## What You Will Build

A complete, production-grade, real-time collaborative project management platform called
**TaskForge**. This is the culmination of the entire course. There is no starter code beyond the
test suite and database schema -- you build everything from scratch.

TaskForge is a backend API that supports multiple users collaborating on projects and tasks, with
real-time WebSocket notifications, background job processing, authentication, caching, rate
limiting, and full observability.

## Requirements

You must implement **all** of the following:

### 1. REST API (axum)

- Projects: full CRUD (create, read, update, delete)
- Tasks: full CRUD with status transitions (todo, in_progress, done) and priority levels
  (low, medium, high, critical)
- Comments on tasks (stretch goal if time permits)
- Pagination on list endpoints (`?page=1&per_page=20`)
- Filtering on list endpoints (e.g., `?status=todo&priority=high`)

### 2. PostgreSQL Persistence (sqlx)

- Migrations are provided in `migrations/001_schema.sql` -- do not modify them
- Implement a repository layer that encapsulates all SQL queries
- Use parameterized queries (no string interpolation of user input)
- Connection pooling via sqlx::PgPool

### 3. JWT Authentication

- `POST /api/auth/register` -- create account with email, password, display name
- `POST /api/auth/login` -- returns access token (short-lived) and refresh token
- `POST /api/auth/refresh` -- exchange refresh token for new access token
- Passwords hashed with argon2
- Role-based access control:
  - **Admin**: can manage all resources, access admin-only routes
  - **Member**: can create/modify own projects and tasks
  - **Viewer**: read-only access, cannot create or modify resources

### 4. WebSocket Real-Time Updates

- Endpoint: `GET /ws?token=<jwt>&project_id=<uuid>`
- Clients subscribe to a project channel
- When a task is created, updated, or deleted within that project, all subscribed clients
  receive a JSON notification
- Notifications include event type, entity data, and timestamp
- Support multiple clients per project and multiple projects simultaneously

### 5. Background Job Processing

- Jobs are stored in the `jobs` database table
- A background worker polls for pending jobs and processes them
- Job types include at minimum: `notification` (e.g., task assigned)
- Failed jobs are retried up to `max_attempts` times
- Jobs transition through statuses: pending -> running -> completed/failed

### 6. Caching

- In-memory cache (moka) for read-heavy endpoints
- Cache task lists and project details
- Invalidate cache on writes (create, update, delete)
- Cache TTL should be configurable

### 7. Rate Limiting

- Token bucket algorithm implemented as tower middleware
- Configurable per-endpoint limits
- Returns HTTP 429 Too Many Requests when exceeded
- Rate limit headers in responses (X-RateLimit-Limit, X-RateLimit-Remaining)

### 8. Observability

- **Structured logging**: use `tracing` with JSON output, include request IDs
- **Prometheus metrics**: expose `/metrics` endpoint with request counts, latencies,
  active connections
- **Health checks**:
  - `GET /health/live` -- returns 200 if process is alive
  - `GET /health/ready` -- returns 200 if database is connected and migrations applied
- **Trace ID header**: every response includes `X-Request-Id`

### 9. Graceful Shutdown

- Handle SIGTERM signal
- Stop accepting new connections
- Drain existing connections (with a timeout)
- Complete in-flight requests
- Shut down background workers cleanly

### 10. Configuration

- Environment-based configuration using the `config` crate
- Required variables: `DATABASE_URL`, `JWT_SECRET`
- Optional with defaults: `HOST` (0.0.0.0), `PORT` (3000), `CACHE_TTL_SECS` (300),
  `RATE_LIMIT_RPS` (100)
- Support `.env` files for local development

### 11. Docker

- Multi-stage Dockerfile:
  - Builder stage: compile release binary
  - Runtime stage: minimal image (e.g., debian-slim) with just the binary
- The image should be runnable with `docker run -e DATABASE_URL=... -p 3000:3000 taskforge`

## Evaluation Criteria

1. **All acceptance tests pass** -- run with `cargo test --test acceptance_tests`
2. **Code quality** -- compiles with zero warnings, passes `cargo clippy --all-targets`,
   passes `cargo fmt --check`
3. **DESIGN.md** -- you must include a `DESIGN.md` file documenting:
   - High-level architecture diagram (ASCII or description)
   - Data model and relationships
   - API endpoint listing with request/response shapes
   - Key design decisions and trade-offs
4. **No unsafe code** unless justified with a comment explaining why
5. **Test coverage** -- in addition to the acceptance tests, include your own unit tests
   and integration tests

## Architecture Guidance

These are suggestions, not requirements. You are free to structure your code however you like
as long as the tests pass.

### Suggested Layer Architecture

```
HTTP Request
    |
    v
Handlers (axum extractors, request/response types)
    |
    v
Services (business logic, validation, authorization)
    |
    v
Repositories (database queries, sqlx)
    |
    v
PostgreSQL
```

### Suggested Shared State

```rust
struct AppState {
    db: PgPool,
    cache: moka::future::Cache<String, serde_json::Value>,
    config: AppConfig,
    ws_channels: DashMap<Uuid, broadcast::Sender<WsEvent>>,
}
```

Pass `Arc<AppState>` to handlers via axum's State extractor.

### WebSocket Approach

- Use `tokio::sync::broadcast` channels, one per project
- Store channels in a `DashMap<Uuid, broadcast::Sender<WsEvent>>`
- On task mutation, look up the project's channel and send an event
- WebSocket handler subscribes to the channel and forwards events to the client

### Background Jobs Approach

- Option A: Database polling -- a tokio task runs a loop, queries for pending jobs,
  processes them, updates status
- Option B: In-memory channel -- tasks publish to a channel, worker receives and processes,
  with database as fallback for crash recovery

## Getting Started

1. Set up PostgreSQL (locally or via Docker)
2. Run the migration: `psql $DATABASE_URL < migrations/001_schema.sql`
3. Copy the environment template: create a `.env` file with at minimum `DATABASE_URL` and
   `JWT_SECRET`
4. Start building your implementation in `src/`
5. Run the acceptance tests: `TEST_SERVER_ADDR=http://127.0.0.1:3000 cargo test --test acceptance_tests`

## API Quick Reference

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | /api/auth/register | No | Create account |
| POST | /api/auth/login | No | Get tokens |
| POST | /api/auth/refresh | No | Refresh access token |
| GET | /api/projects | Yes | List projects (paginated) |
| POST | /api/projects | Yes | Create project |
| GET | /api/projects/:id | Yes | Get project |
| PUT | /api/projects/:id | Yes | Update project |
| DELETE | /api/projects/:id | Yes | Delete project |
| GET | /api/projects/:id/tasks | Yes | List tasks (filtered, paginated) |
| POST | /api/projects/:id/tasks | Yes | Create task |
| GET | /api/tasks/:id | Yes | Get task |
| PUT | /api/tasks/:id | Yes | Update task |
| DELETE | /api/tasks/:id | Yes | Delete task |
| GET | /ws | Yes (query param) | WebSocket connection |
| GET | /health/live | No | Liveness check |
| GET | /health/ready | No | Readiness check |
| GET | /metrics | No | Prometheus metrics |

Good luck. This is the hardest project in the course -- and the most rewarding.

---

### Career Checkpoint

When you finish the capstone, **deploy it**. Get a live URL. This is your
flagship portfolio piece — the project you'll talk about in every interview.
In Phase 9 you'll polish the README and write about it, but having a running
deployment from day one demonstrates that you don't just write code, you ship
products. Use Fly.io, Railway, Render, or Shuttle — all have free tiers that
work for this.
