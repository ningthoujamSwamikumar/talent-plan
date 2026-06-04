use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::Role;

/// JWT claims embedded in access tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — the user's UUID.
    pub sub: String,
    /// User email.
    pub email: String,
    /// User role.
    pub role: String,
    /// Issued-at timestamp (Unix epoch seconds).
    pub iat: i64,
    /// Expiry timestamp (Unix epoch seconds).
    pub exp: i64,
}

/// Configuration for JWT token generation.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// Secret key used for HMAC-SHA256 signing.
    pub secret: String,
    /// Access token lifetime in minutes.
    pub access_token_minutes: i64,
    /// Refresh token lifetime in days.
    pub refresh_token_days: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: "default-secret-change-in-production".to_string(),
            access_token_minutes: 15,
            refresh_token_days: 7,
        }
    }
}

/// Create a signed JWT access token for the given user.
///
/// The token contains the user's id, email, and role as claims and expires
/// after the configured number of minutes.
pub fn create_access_token(
    user_id: Uuid,
    email: &str,
    role: &Role,
    config: &JwtConfig,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::minutes(config.access_token_minutes);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        iat: now.timestamp(),
        exp: exp.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to create access token: {}", e)))
}

/// Create a refresh token string.
///
/// This generates a random UUID-based token. The caller is responsible for
/// hashing it before storing in the database.
pub fn create_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

/// Validate and decode a JWT access token.
///
/// Returns the decoded claims on success, or an `AppError` if the token is
/// invalid, expired, or tampered with.
pub fn validate_token(token: &str, config: &JwtConfig) -> Result<TokenData<Claims>, AppError> {
    let validation = Validation::default();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )
    .map_err(AppError::from)
}

/// Extract user id from validated claims.
pub fn user_id_from_claims(claims: &Claims) -> Result<Uuid, AppError> {
    Uuid::parse_str(&claims.sub)
        .map_err(|e| AppError::InvalidToken(format!("Invalid user id in token: {}", e)))
}

/// Extract role from validated claims.
pub fn role_from_claims(claims: &Claims) -> Result<Role, AppError> {
    claims
        .role
        .parse::<Role>()
        .map_err(|e| AppError::InvalidToken(format!("Invalid role in token: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JwtConfig {
        JwtConfig {
            secret: "test-secret-key".to_string(),
            access_token_minutes: 15,
            refresh_token_days: 7,
        }
    }

    #[test]
    fn test_create_and_validate_token() {
        let config = test_config();
        let user_id = Uuid::new_v4();

        let token =
            create_access_token(user_id, "test@example.com", &Role::Member, &config).unwrap();
        let decoded = validate_token(&token, &config).unwrap();

        assert_eq!(decoded.claims.sub, user_id.to_string());
        assert_eq!(decoded.claims.email, "test@example.com");
        assert_eq!(decoded.claims.role, "member");
    }

    #[test]
    fn test_invalid_secret_rejects_token() {
        let config = test_config();
        let user_id = Uuid::new_v4();

        let token =
            create_access_token(user_id, "test@example.com", &Role::Member, &config).unwrap();

        let bad_config = JwtConfig {
            secret: "wrong-secret".to_string(),
            ..config
        };

        assert!(validate_token(&token, &bad_config).is_err());
    }

    #[test]
    fn test_refresh_token_is_unique() {
        let t1 = create_refresh_token();
        let t2 = create_refresh_token();
        assert_ne!(t1, t2);
    }
}
