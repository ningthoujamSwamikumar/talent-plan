use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;
use validator::Validate;

use crate::auth::{jwt, password};
use crate::error::AppError;
use crate::models::{
    ApiKeyCreatedResponse, AuthResponse, CreateApiKeyRequest, CreateUser, LoginRequest,
    RefreshRequest, Role, UserResponse,
};
use crate::repository::users::UserRepository;

/// Application state shared across handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    pub user_repo: UserRepository,
    pub jwt_config: jwt::JwtConfig,
}

// ---------------------------------------------------------------------------
// POST /auth/register
// ---------------------------------------------------------------------------

/// Register a new user account.
///
/// Validates the request, hashes the password, creates the user record, and
/// returns the sanitized user profile.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // Validate request body
    payload.validate().map_err(|e| {
        AppError::ValidationError(format!("Validation failed: {}", e))
    })?;

    // Check for existing user
    if state.user_repo.get_by_email(&payload.email).await?.is_some() {
        return Err(AppError::Conflict(
            "A user with this email already exists".into(),
        ));
    }

    // Hash password
    let password_hash = password::hash_password(&payload.password)?;

    // Create user
    let user = state
        .user_repo
        .create_user(&payload.email, &password_hash, &payload.display_name, Role::Member)
        .await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

// ---------------------------------------------------------------------------
// POST /auth/login
// ---------------------------------------------------------------------------

/// Authenticate with email and password.
///
/// On success, returns an access token (JWT) and a refresh token.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Look up user
    let user = state
        .user_repo
        .get_by_email(&payload.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // Check account lockout
    if let Some(locked_until) = user.locked_until {
        if chrono::Utc::now() < locked_until {
            return Err(AppError::AccountLocked);
        }
    }

    // Verify password
    if !password::verify_password(&payload.password, &user.password_hash)? {
        // TODO: increment failed_attempts in database
        return Err(AppError::InvalidCredentials);
    }

    // TODO: reset failed_attempts to 0

    // Generate tokens
    let access_token =
        jwt::create_access_token(user.id, &user.email, &user.role, &state.jwt_config)?;
    let refresh_token = jwt::create_refresh_token();

    // Store refresh token hash in database
    let refresh_hash = crate::auth::api_key::hash_api_key(&refresh_token);
    state
        .user_repo
        .store_refresh_token(user.id, &refresh_hash)
        .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.jwt_config.access_token_minutes * 60,
        user: UserResponse::from(user),
    }))
}

// ---------------------------------------------------------------------------
// POST /auth/refresh
// ---------------------------------------------------------------------------

/// Exchange a valid refresh token for a new access + refresh token pair.
///
/// Implements token rotation: the old refresh token is invalidated.
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let token_hash = crate::auth::api_key::hash_api_key(&payload.refresh_token);

    // Validate and consume the refresh token
    let user_id = state
        .user_repo
        .validate_refresh_token(&token_hash)
        .await?
        .ok_or(AppError::Unauthorized("Invalid refresh token".into()))?;

    // Look up user
    let user = state
        .user_repo
        .get_by_id(user_id)
        .await?
        .ok_or(AppError::Unauthorized("User not found".into()))?;

    // Generate new token pair
    let access_token =
        jwt::create_access_token(user.id, &user.email, &user.role, &state.jwt_config)?;
    let new_refresh_token = jwt::create_refresh_token();

    // Store new refresh token
    let new_hash = crate::auth::api_key::hash_api_key(&new_refresh_token);
    state
        .user_repo
        .store_refresh_token(user.id, &new_hash)
        .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.jwt_config.access_token_minutes * 60,
        user: UserResponse::from(user),
    }))
}

// ---------------------------------------------------------------------------
// POST /auth/api-keys
// ---------------------------------------------------------------------------

/// Create a new API key for the authenticated user.
///
/// Returns the plaintext key exactly once. The key is stored hashed in the
/// database.
pub async fn create_api_key(
    State(state): State<AppState>,
    auth: crate::auth::middleware::AuthUser,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiKeyCreatedResponse>), AppError> {
    payload.validate().map_err(|e| {
        AppError::ValidationError(format!("Validation failed: {}", e))
    })?;

    let (plaintext_key, key_prefix) = crate::auth::api_key::generate_api_key();
    let key_hash = crate::auth::api_key::hash_api_key(&plaintext_key);

    let expires_at = payload.expires_in_days.map(|days| {
        chrono::Utc::now() + chrono::Duration::days(days)
    });

    let api_key = state
        .user_repo
        .create_api_key(auth.user_id, &payload.name, &key_hash, &key_prefix, expires_at)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiKeyCreatedResponse {
            id: api_key.id,
            name: api_key.name,
            key: plaintext_key,
            key_prefix: api_key.key_prefix,
            expires_at: api_key.expires_at,
            created_at: api_key.created_at,
        }),
    ))
}

// ---------------------------------------------------------------------------
// DELETE /auth/api-keys/:id
// ---------------------------------------------------------------------------

/// Revoke an API key belonging to the authenticated user.
pub async fn revoke_api_key(
    State(state): State<AppState>,
    auth: crate::auth::middleware::AuthUser,
    axum::extract::Path(key_id): axum::extract::Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state
        .user_repo
        .revoke_api_key(auth.user_id, key_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
