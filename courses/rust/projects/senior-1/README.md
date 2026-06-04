# Phase 6, Project 1: API Design and Versioning

In this project, you will learn how to design, document, version, and evolve a REST API
professionally. You will use OpenAPI (via `utoipa`) to generate machine-readable specs,
serve interactive documentation with Swagger UI, implement URL-based versioning so that
multiple API versions coexist, evolve your schema without breaking existing clients, signal
deprecation through standard HTTP headers, and generate a type-safe client SDK from your
OpenAPI specification.

By the end of this project you will understand the full lifecycle of an API: design,
documentation, versioning, evolution, deprecation, and client generation.

## Prerequisites

- Completion of Phase 5 projects (production Rust web services)
- Familiarity with axum handlers, extractors, and routing
- Basic understanding of JSON serialization with serde

## Project Overview

You are building a Task Management API. The API manages tasks with fields like `id`, `title`,
`description`, `status`, `priority`, `created_at`, and `updated_at`. Over the course of this
project you will evolve the API from v1 to v2, keeping both versions running simultaneously.

## Part 1: OpenAPI with utoipa

**Goal:** Annotate your existing endpoints so that `utoipa` can generate a complete OpenAPI 3.0
specification automatically.

**Tasks:**
1. Define your `Task` model in `src/lib.rs` with `#[derive(utoipa::ToSchema)]`
2. Annotate each handler in `src/v1/handlers.rs` with `#[utoipa::path(...)]`
3. Create an `ApiDoc` struct in `src/openapi.rs` using the `#[derive(OpenApi)]` macro that
   collects all paths and schemas
4. Add a `/api-doc/openapi.json` endpoint that serves the generated JSON spec
5. Verify the spec is valid by inspecting the JSON output

**Hints:**
- `utoipa::ToSchema` works alongside `serde::Serialize` / `serde::Deserialize`
- Use `#[utoipa::path(get, path = "/v1/tasks", responses(...))]` syntax
- The `tag` attribute groups related endpoints in the documentation

## Part 2: Swagger UI

**Goal:** Serve an interactive Swagger UI so that developers can explore and test the API
directly from a browser.

**Tasks:**
1. Add `utoipa-swagger-ui` with the `axum` feature
2. Mount the Swagger UI at `/swagger-ui/` using `SwaggerUi::new(...)`
3. Configure it to load your OpenAPI spec from `/api-doc/openapi.json`
4. Verify you can open the UI in a browser and execute requests

**Hints:**
- `SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", openapi)` is the pattern
- Make sure the Swagger UI route is merged into your main router

## Part 3: URL Path Versioning

**Goal:** Implement URL-based API versioning so that `/v1/tasks` and `/v2/tasks` coexist
in the same application.

**Tasks:**
1. Create `src/v1/handlers.rs` with the original task endpoints (CRUD)
2. Create `src/v2/handlers.rs` with an enhanced version that adds a `tags` field and
   a `due_date` field to tasks
3. In `src/lib.rs`, build a router that nests v1 handlers under `/v1` and v2 handlers
   under `/v2`
4. Each version uses its own model types (`v1::Task`, `v2::Task`)
5. Both versions share the same in-memory storage (using a shared `Arc<RwLock<...>>`)

**Hints:**
- Use `axum::Router::nest("/v1", v1_router).nest("/v2", v2_router)`
- The shared state should store the superset of fields; v1 handlers simply omit the new fields
  when serializing responses

## Part 4: Backward-Compatible Evolution

**Goal:** Demonstrate that v1 clients continue to work after v2 is deployed.

**Tasks:**
1. When a task is created via v1, the new v2 fields (`tags`, `due_date`) get default values
2. When a task is read via v1, the v2 fields are omitted from the response
3. When a task created via v2 (with tags and due_date) is read via v1, only v1 fields appear
4. Write tests that create tasks via v2 and read them via v1 to confirm compatibility
5. Implement a `TaskStorage` trait so that both versions can share the same backing store

**Hints:**
- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- The `From<InternalTask>` trait is useful for converting between internal and versioned types

## Part 5: Deprecation Headers

**Goal:** Signal to clients that v1 is deprecated and will be sunset on a specific date.

**Tasks:**
1. Create an axum middleware layer that adds deprecation headers to all `/v1/*` responses
2. Add the `Sunset` header with a date (e.g., `Sat, 31 Dec 2025 23:59:59 GMT`)
3. Add the `Deprecation` header with the value `true`
4. Add a `Link` header pointing to the v2 documentation
5. Include a `Warning` header with a human-readable deprecation message
6. Write tests that verify all deprecation headers are present on v1 responses but absent
   on v2 responses

**Hints:**
- Use `axum::middleware::from_fn` or `tower_http::set_header::SetResponseHeaderLayer`
- Headers can be added as a layer applied to the v1 sub-router only

## Part 6: SDK Generation

**Goal:** Generate a type-safe Rust client from the OpenAPI specification.

**Tasks:**
1. Write a function that serializes the `ApiDoc` to a JSON file
2. Document how to use `openapi-generator` or `progenitor` to generate a Rust client
3. Create a simple integration-style test (in `tests/versioning_tests.rs`) that uses `reqwest`
   to exercise both v1 and v2 endpoints, simulating what a generated client would do
4. Verify that the generated client types match the schema

**Note:** Full code generation is optional; the focus is understanding how the OpenAPI spec
enables it. The integration tests should demonstrate the contract.

## Testing

Run all tests:
```
cargo test
```

The test suite in `tests/versioning_tests.rs` covers:
- OpenAPI spec generation and validity
- v1 CRUD operations
- v2 CRUD operations with new fields
- Cross-version compatibility (create in v2, read in v1)
- Deprecation headers on v1 responses
- Swagger UI availability
- Content negotiation

## What You Will Learn

- How to document APIs with OpenAPI/Swagger using utoipa
- URL-based API versioning strategies
- Backward-compatible API evolution patterns
- Deprecation and sunset communication via HTTP headers
- SDK generation from API specifications
- Managing multiple API versions in a single codebase
