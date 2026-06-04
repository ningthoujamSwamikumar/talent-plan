use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Unified error type that bridges sqlx, validation, and HTTP concerns.
#[derive(Debug)]
pub enum AppError {
    /// A row the client asked for does not exist.
    NotFound(String),
    /// Request payload failed validation.
    Validation(String),
    /// A database constraint was violated (FK, unique, check, etc.).
    Conflict(String),
    /// Catch-all for unexpected sqlx / internal errors.
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "not found: {msg}"),
            AppError::Validation(msg) => write!(f, "validation: {msg}"),
            AppError::Conflict(msg) => write!(f, "conflict: {msg}"),
            AppError::Internal(msg) => write!(f, "internal: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

// ---- Conversions ----------------------------------------------------------

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => AppError::NotFound("resource not found".into()),
            sqlx::Error::Database(db_err) => {
                // PostgreSQL error codes
                let code = db_err.code().unwrap_or_default();
                match code.as_ref() {
                    // 23503 = foreign_key_violation
                    "23503" => AppError::Conflict(format!("foreign key violation: {db_err}")),
                    // 23505 = unique_violation
                    "23505" => AppError::Conflict(format!("unique constraint violation: {db_err}")),
                    // 23514 = check_violation
                    "23514" => AppError::Validation(format!("check constraint violation: {db_err}")),
                    _ => AppError::Internal(format!("database error: {db_err}")),
                }
            }
            _ => AppError::Internal(format!("database error: {err}")),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::Validation(err.to_string())
    }
}

// ---- Axum response --------------------------------------------------------

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Internal(msg) => {
                tracing::error!("internal error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".into(),
                )
            }
        };

        let body = json!({ "error": message });
        (status, Json(body)).into_response()
    }
}
