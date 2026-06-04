# Building Block Web-1: HTTP APIs and the Axum Framework

**Prerequisites**: Complete Phase 0 and Phase 1 (Projects 1-5).

Before starting [Project: REST API with Axum](../projects/web-1/README.md), complete
the readings and exercises below.

## What to read

- [Axum documentation and examples](https://docs.rs/axum/latest/axum/).
  Read the module-level docs carefully. Axum's documentation is excellent and serves
  as both reference and tutorial.

- [Axum Getting Started Guide](https://docs.rs/axum/latest/axum/#example).
  Build the example from the docs before starting the project.

- [Tower Service Trait Explained](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait).
  Tower is the middleware framework underlying axum. Understanding `Service` helps
  you reason about middleware, layers, and the request/response pipeline.

- [HTTP API Design Guide](https://github.com/interagent/http-api-design).
  Best practices for RESTful API design: resource naming, status codes, pagination,
  error responses.

- [Validator crate documentation](https://docs.rs/validator/latest/validator/).
  How to validate request data before processing.

## Key concepts

### Axum Architecture

```
Request → Router → Middleware (Tower Layers) → Handler → Response
                                                  ↓
                                              Extractors
                                           (parse request parts)
```

**Router**: Maps HTTP methods and paths to handlers.
**Handler**: An async function that takes extractors and returns a response.
**Extractor**: Parses parts of the request (body, path, query, headers, state).
**Layer**: Wraps handlers with cross-cutting concerns (logging, CORS, compression).

### Common Extractors

```rust
use axum::extract::{State, Path, Query, Json};

async fn create_task(
    State(state): State<AppState>,          // shared application state
    Path(project_id): Path<Uuid>,           // from URL path
    Query(params): Query<PaginationParams>, // from query string
    Json(body): Json<CreateTaskRequest>,    // from request body
) -> impl IntoResponse {
    // ...
}
```

### Status Codes

| Operation | Success | Error |
|-----------|---------|-------|
| Create | 201 Created | 400/422 Validation |
| Read | 200 OK | 404 Not Found |
| Update | 200 OK | 404 Not Found |
| Delete | 204 No Content | 404 Not Found |
| Any | — | 500 Internal Error |

## Exercises

**Exercise 1**: Create a minimal axum server with three routes:
- `GET /` returns `"Hello, World!"`
- `GET /greet/:name` returns `"Hello, {name}!"`
- `POST /echo` returns the JSON body back as-is

**Exercise 2**: Add a `State` extractor with a counter (Arc<Mutex<u64>>).
Create `GET /count` that returns the current count and `POST /count` that
increments it.

**Exercise 3**: Add `tower-http::trace::TraceLayer` to your server and observe
the structured log output when you make requests.

## You're ready when...

- [ ] You can create axum routes with different HTTP methods
- [ ] You understand extractors and how they parse requests
- [ ] You can return JSON responses with proper status codes
- [ ] You understand how tower layers wrap handlers

Next: [Project: REST API with Axum](../projects/web-1/README.md)
