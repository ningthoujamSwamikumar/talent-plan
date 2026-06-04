use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Application error types covering both general and auth-specific errors.
#[derive(Debug)]
pub enum AppError {
    // General errors
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Conflict(String),
    ValidationError(String),

    // Auth-specific errors
    Unauthorized(String),
    Forbidden(String),
    InvalidCredentials,
    TokenExpired,
    InvalidToken(String),
    AccountLocked,

    // Database errors
    Database(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::InvalidCredentials => write!(f, "Invalid email or password"),
            AppError::TokenExpired => write!(f, "Token has expired"),
            AppError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AppError::AccountLocked => write!(f, "Account is temporarily locked"),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid email or password".into())
            }
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token has expired".into()),
            AppError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::AccountLocked => (
                StatusCode::TOO_MANY_REQUESTS,
                "Account is temporarily locked due to too many failed attempts".into(),
            ),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = json!({
            "error": {
                "status": status.as_u16(),
                "message": message,
            }
        });

        (status, Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        AppError::Database("An internal database error occurred".into())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken(format!("Token validation failed: {}", err)),
        }
    }
}
