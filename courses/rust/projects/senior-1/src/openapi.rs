//! OpenAPI specification generation.
//!
//! This module defines the `ApiDoc` struct that collects all paths and schemas
//! from both v1 and v2 handlers, and provides functions to serve the spec and
//! Swagger UI.

use axum::{Json, Router};

/// The OpenAPI document struct.
///
/// TODO: Students annotate this with `#[derive(utoipa::OpenApi)]` and list all
/// paths and schemas from v1 and v2 handlers.
///
/// Example:
/// ```ignore
/// #[derive(utoipa::OpenApi)]
/// #[openapi(
///     paths(
///         crate::v1::handlers::list_tasks,
///         crate::v1::handlers::get_task,
///         crate::v1::handlers::create_task,
///         crate::v1::handlers::update_task,
///         crate::v1::handlers::delete_task,
///         crate::v2::handlers::list_tasks,
///         crate::v2::handlers::get_task,
///         crate::v2::handlers::create_task,
///         crate::v2::handlers::update_task,
///         crate::v2::handlers::delete_task,
///     ),
///     components(schemas(
///         crate::TaskStatus,
///         crate::Priority,
///         crate::v1::handlers::TaskV1,
///         crate::v1::handlers::CreateTaskV1,
///         crate::v1::handlers::UpdateTaskV1,
///         crate::v2::handlers::TaskV2,
///         crate::v2::handlers::CreateTaskV2,
///         crate::v2::handlers::UpdateTaskV2,
///     )),
///     tags(
///         (name = "tasks-v1", description = "Task API v1"),
///         (name = "tasks-v2", description = "Task API v2 — includes tags and due_date"),
///     )
/// )]
/// pub struct ApiDoc;
/// ```
pub struct ApiDoc;

/// Handler that returns the OpenAPI JSON spec.
///
/// TODO: Use `utoipa::OpenApi::openapi()` to generate the spec and return it as JSON.
pub async fn openapi_spec() -> Json<serde_json::Value> {
    todo!("Return the OpenAPI spec as JSON")
}

/// Build a router that serves both the OpenAPI spec and Swagger UI.
///
/// TODO: Mount:
/// - GET `/api-doc/openapi.json` -> `openapi_spec`
/// - Swagger UI at `/swagger-ui/` loading from `/api-doc/openapi.json`
pub fn openapi_router() -> Router {
    todo!("Build the OpenAPI + Swagger UI router")
}

/// Write the OpenAPI spec to a JSON file on disk.
///
/// This is useful for feeding into SDK generators like `openapi-generator` or `progenitor`.
///
/// TODO: Serialize the OpenAPI doc and write it to `path`.
pub fn write_spec_to_file(_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    todo!("Write the OpenAPI spec to a file")
}
