use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{ApiKey, Role, User};

/// Repository for user-related database operations.
///
/// In a real application this would hold a `sqlx::PgPool`. For this project
/// skeleton, methods are stubbed to define the interface students must
/// implement.
#[derive(Debug, Clone)]
pub struct UserRepository {
    // TODO: add `pool: sqlx::PgPool` when connecting to a real database.
    _private: (),
}

impl UserRepository {
    /// Create a new `UserRepository`.
    ///
    /// Students should accept a `sqlx::PgPool` here.
    pub fn new() -> Self {
        UserRepository { _private: () }
    }

    // -----------------------------------------------------------------------
    // User CRUD
    // -----------------------------------------------------------------------

    /// Insert a new user and return the created record.
    pub async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
        display_name: &str,
        role: Role,
    ) -> Result<User, AppError> {
        // TODO: INSERT INTO users (...) VALUES (...) RETURNING *
        let _ = (email, password_hash, display_name, role);
        Err(AppError::Internal(
            "UserRepository::create_user not yet implemented".into(),
        ))
    }

    /// Look up a user by email address. Returns `None` if not found.
    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        // TODO: SELECT * FROM users WHERE email = $1
        let _ = email;
        Ok(None)
    }

    /// Look up a user by id. Returns `None` if not found.
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        // TODO: SELECT * FROM users WHERE id = $1
        let _ = id;
        Ok(None)
    }

    // -----------------------------------------------------------------------
    // Refresh tokens
    // -----------------------------------------------------------------------

    /// Store a hashed refresh token associated with a user.
    pub async fn store_refresh_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
    ) -> Result<(), AppError> {
        // TODO: INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        //       VALUES ($1, $2, NOW() + INTERVAL '7 days')
        let _ = (user_id, token_hash);
        Ok(())
    }

    /// Validate a refresh token hash. If valid, revoke it and return the
    /// associated user id (token rotation).
    pub async fn validate_refresh_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<Uuid>, AppError> {
        // TODO:
        // 1. SELECT user_id FROM refresh_tokens
        //    WHERE token_hash = $1 AND revoked = FALSE AND expires_at > NOW()
        // 2. UPDATE refresh_tokens SET revoked = TRUE WHERE token_hash = $1
        // 3. Return the user_id
        let _ = token_hash;
        Ok(None)
    }

    // -----------------------------------------------------------------------
    // API keys
    // -----------------------------------------------------------------------

    /// Create and store a new API key record.
    pub async fn create_api_key(
        &self,
        user_id: Uuid,
        name: &str,
        key_hash: &str,
        key_prefix: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey, AppError> {
        // TODO: INSERT INTO api_keys (...) VALUES (...) RETURNING *
        let _ = (user_id, name, key_hash, key_prefix, expires_at);
        Err(AppError::Internal(
            "UserRepository::create_api_key not yet implemented".into(),
        ))
    }

    /// Look up an API key by its hash. Returns `None` if not found or revoked.
    pub async fn get_api_key_by_hash(&self, key_hash: &str) -> Result<Option<ApiKey>, AppError> {
        // TODO: SELECT * FROM api_keys
        //       WHERE key_hash = $1 AND revoked = FALSE
        //       AND (expires_at IS NULL OR expires_at > NOW())
        let _ = key_hash;
        Ok(None)
    }

    /// Revoke an API key. Only the owner or an admin should call this.
    pub async fn revoke_api_key(&self, user_id: Uuid, key_id: Uuid) -> Result<(), AppError> {
        // TODO: UPDATE api_keys SET revoked = TRUE
        //       WHERE id = $1 AND user_id = $2
        let _ = (user_id, key_id);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Account lockout helpers
    // -----------------------------------------------------------------------

    /// Increment the failed login attempts counter.
    pub async fn increment_failed_attempts(&self, user_id: Uuid) -> Result<(), AppError> {
        // TODO: UPDATE users SET failed_attempts = failed_attempts + 1 WHERE id = $1
        // If failed_attempts >= 5, SET locked_until = NOW() + INTERVAL '15 minutes'
        let _ = user_id;
        Ok(())
    }

    /// Reset the failed login attempts counter after a successful login.
    pub async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), AppError> {
        // TODO: UPDATE users SET failed_attempts = 0, locked_until = NULL WHERE id = $1
        let _ = user_id;
        Ok(())
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}
