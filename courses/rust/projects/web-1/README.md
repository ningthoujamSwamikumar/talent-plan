# Project: REST API with Axum

Phase 2, Project 1 of the PNA Rust course.

## Introduction

### Why Axum?

[Axum](https://github.com/tokio-rs/axum) is a web application framework built on top of
[Tower](https://github.com/tower-rs/tower) and [Hyper](https://github.com/hyperium/hyper).
It leverages Rust's type system to provide compile-time guarantees about your web handlers,
extractors, and middleware. Because axum is built on Tower, every piece of middleware in
the Tower ecosystem is available to you. This composability is one of axum's greatest
strengths: you can add tracing, rate limiting, compression, CORS, and authentication as
reusable layers without modifying your handler logic.

### What You Will Build

In this project you will build **TaskForge**, a RESTful task management API. The API
supports:

- **Projects** -- top-level containers for related work.
- **Tasks** -- individual work items that belong to a project, each with a status and
  priority.

Data is stored in memory using a `HashMap` behind `Arc<RwLock<...>>`. There is no
database; the focus is on learning HTTP semantics, routing, extractors, error handling,
and middleware.

By the end you will have a fully tested JSON API with proper status codes, validation,
pagination, filtering, and middleware.

---

## Part 1: Hello Axum

**Goal:** Start the server and respond to your first request.

Create a `Router` with a single `GET /health` route that returns `200 OK` with a JSON
body:

```json
{ "status": "ok" }
```

### Key Concepts

- **Router** -- the central type that maps paths to handlers. Routes are added with
  `.route("/path", get(handler))`.
- **Handlers** -- async functions whose arguments are *extractors* and whose return type
  implements `IntoResponse`.
- **Extractors** -- types that axum pulls out of the incoming request for you. Examples:
  `Json<T>`, `Path<T>`, `Query<T>`, `State<T>`.
- **`IntoResponse`** -- any type that can be turned into an HTTP response. Tuples like
  `(StatusCode, Json<T>)` work out of the box.

### Exercises

1. Implement the `app()` function in `src/lib.rs` so it returns a `Router` with the
   health route.
2. Implement the health handler in `src/handlers/health.rs`.
3. Run `cargo test health_check` and make it pass.

---

## Part 2: Data Models

**Goal:** Define the domain types that flow through the API.

### Structs

| Type | Fields |
|------|--------|
| `Project` | `id`, `name`, `description`, `created_at`, `updated_at` |
| `Task` | `id`, `project_id`, `title`, `description`, `status`, `priority`, `created_at`, `updated_at` |

### Enums

- `TaskStatus` -- `Todo`, `InProgress`, `Done`
- `Priority` -- `Low`, `Medium`, `High`, `Critical`

### Request Types

- `CreateProject { name, description }` -- `name` must not be empty.
- `UpdateProject { name, description }` -- both optional.
- `CreateTask { title, description, status, priority }` -- `title` must not be empty.
- `UpdateTask { title, description, status, priority }` -- all optional.

### Validation

Use the `validator` crate with `#[derive(Validate)]` and `#[validate(length(min = 1))]`
on required string fields. When a request fails validation, return a `422 Unprocessable
Entity` response with a JSON body describing the errors.

### Pagination

Define generic helpers:

- `PaginationParams { page: Option<u32>, per_page: Option<u32> }` -- extracted from query
  parameters.
- `PaginatedResponse<T> { data: Vec<T>, total: usize, page: u32, per_page: u32 }` --
  returned in list endpoints.

### Exercises

1. Implement all structs and enums in `src/models.rs`.
2. Make sure `cargo build` succeeds.

---

## Part 3: In-Memory Store

**Goal:** Build a thread-safe data store using `Arc<RwLock<HashMap>>`.

### `AppState`

```
AppState {
    projects: RwLock<HashMap<Uuid, Project>>,
    tasks: RwLock<HashMap<Uuid, Task>>,
}
```

The state is wrapped in `Arc` before being passed to the router via `.with_state()`. Axum
clones the `Arc` into each handler through the `State` extractor.

### Methods

Implement CRUD methods on `AppState`:

- `create_project`, `get_project`, `list_projects`, `update_project`, `delete_project`
- `create_task`, `get_task`, `list_tasks`, `update_task`, `delete_task`

Each method acquires the appropriate lock (`read` or `write`), performs the operation, and
returns a `Result` so errors propagate to the handler.

### Exercises

1. Fill in the `AppState` fields and all method bodies in `src/store.rs`.
2. Run `cargo test` -- the store methods are exercised indirectly by the integration
   tests.

---

## Part 4: CRUD for Projects

**Goal:** Implement the project endpoints.

| Method | Path | Status | Description |
|--------|------|--------|-------------|
| POST | `/projects` | 201 | Create a project |
| GET | `/projects` | 200 | List all projects |
| GET | `/projects/:id` | 200 | Get one project |
| PUT | `/projects/:id` | 200 | Update a project |
| DELETE | `/projects/:id` | 204 | Delete a project |

### Handler Signatures

```rust
async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateProject>,
) -> Result<impl IntoResponse, AppError> { ... }
```

### Exercises

1. Implement handlers in `src/handlers/projects.rs`.
2. Wire routes in `app()`.
3. Run `cargo test` and make all project tests pass.

---

## Part 5: CRUD for Tasks

**Goal:** Implement the task endpoints. Tasks belong to a project.

| Method | Path | Status | Description |
|--------|------|--------|-------------|
| POST | `/projects/:project_id/tasks` | 201 | Create a task in a project |
| GET | `/projects/:project_id/tasks` | 200 | List tasks for a project |
| GET | `/tasks/:id` | 200 | Get one task |
| PUT | `/tasks/:id` | 200 | Update a task |
| DELETE | `/tasks/:id` | 204 | Delete a task |

When creating a task, verify the parent project exists. If not, return `404`.

### Exercises

1. Implement handlers in `src/handlers/tasks.rs`.
2. Wire routes in `app()`.
3. Run `cargo test` and make all task tests pass.

---

## Part 6: Error Handling

**Goal:** Return structured JSON errors with appropriate HTTP status codes.

### `AppError`

Define an enum:

```rust
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    InternalError(String),
}
```

Implement `IntoResponse` so each variant maps to the correct status code and returns:

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Project not found"
  }
}
```

| Variant | Status Code | Code |
|---------|-------------|------|
| `NotFound` | 404 | `NOT_FOUND` |
| `ValidationError` | 422 | `VALIDATION_ERROR` |
| `InternalError` | 500 | `INTERNAL_ERROR` |

### Exercises

1. Implement `AppError` and its `IntoResponse` impl in `src/error.rs`.
2. Use `AppError` as the error type in all handlers.
3. Run `cargo test` and make the error-related tests pass.

---

## Part 7: Pagination and Filtering

**Goal:** Add query parameter support to list endpoints.

### Pagination

`GET /projects/:id/tasks?page=1&per_page=10`

Default: `page = 1`, `per_page = 10`. The response wraps items in `PaginatedResponse`.

### Filtering

`GET /projects/:id/tasks?status=done&priority=high`

Filter tasks by status and/or priority. These are additional optional query parameters.

### Exercises

1. Extend `list_tasks` to accept `PaginationParams` and filter parameters.
2. Implement pagination logic (slice the in-memory vec).
3. Run `cargo test` and make the pagination and filter tests pass.

---

## Part 8: Middleware

**Goal:** Add cross-cutting concerns via Tower layers.

### CORS

Use `tower_http::cors::CorsLayer` to allow requests from any origin during development.
Verify the `access-control-allow-origin` header appears in responses.

### Request Tracing

Use `tower_http::trace::TraceLayer` to log every request with method, path, and response
status.

### Request ID

Write (or use) middleware that adds an `x-request-id` header to every response. This is
useful for debugging.

### Exercises

1. Add `CorsLayer`, `TraceLayer`, and request-id middleware in `app()`.
2. Run `cargo test` and make the CORS test pass.
3. Start the server with `cargo run` and observe trace output in the terminal.

---

## Running the Project

```bash
# Run all tests
cargo test

# Start the server
cargo run

# Try it out
curl http://127.0.0.1:3000/health
curl -X POST http://127.0.0.1:3000/projects \
  -H 'Content-Type: application/json' \
  -d '{"name": "My Project"}'
```

## Going Further

After completing all parts, consider these extensions:

- Add a `Comment` resource nested under tasks.
- Persist data to a JSON file on disk.
- Add authentication with a simple API key middleware.
- Add rate limiting with `tower::limit`.
- Generate OpenAPI documentation with `utoipa`.
