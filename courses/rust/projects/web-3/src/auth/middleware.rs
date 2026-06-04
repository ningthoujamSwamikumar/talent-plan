use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::auth::jwt::{self, JwtConfig};
use crate::error::AppError;
use crate::models::Role;

/// Extractor that validates the JWT from the `Authorization: Bearer <token>`
/// header and provides the authenticated user's identity to handlers.
///
/// If the token is missing, expired, or invalid, the request is rejected with
/// a 401 Unauthorized response before the handler runs.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: Role,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try Bearer token first
        if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            let auth_str = auth_header
                .to_str()
                .map_err(|_| AppError::Unauthorized("Invalid authorization header".into()))?;

            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // Retrieve JWT config from request extensions.
                // In a real app this would come from the application state.
                let config = parts
                    .extensions
                    .get::<JwtConfig>()
                    .cloned()
                    .unwrap_or_default();

                let token_data = jwt::validate_token(token, &config)?;
                let user_id = jwt::user_id_from_claims(&token_data.claims)?;
                let role = jwt::role_from_claims(&token_data.claims)?;

                return Ok(AuthUser {
                    user_id,
                    email: token_data.claims.email,
                    role,
                });
            }
        }

        // Try X-API-Key header as fallback.
        // In a full implementation this would look up the hashed key in the
        // database and resolve the associated user. Here we define the
        // extraction point so that handlers can rely on AuthUser regardless
        // of the authentication method.
        if let Some(api_key_header) = parts.headers.get("X-API-Key") {
            let _api_key = api_key_header
                .to_str()
                .map_err(|_| AppError::Unauthorized("Invalid API key header".into()))?;

            // TODO: Look up hashed API key in database and resolve user.
            // For now, return Unauthorized so callers know the path exists.
            return Err(AppError::Unauthorized(
                "API key authentication not yet connected to database".into(),
            ));
        }

        Err(AppError::Unauthorized(
            "Missing authentication token".into(),
        ))
    }
}

/// Role-guard extractor. Use this alongside `AuthUser` to enforce a minimum
/// role level on a route.
///
/// ```ignore
/// async fn admin_only(
///     auth: AuthUser,
///     _guard: RequireRole<{ Role::Admin as u8 }>,
/// ) -> impl IntoResponse { ... }
/// ```
///
/// Because const generics over enums are not yet stable, we use a simpler
/// runtime approach: store the minimum role and check it during extraction.
#[derive(Debug, Clone)]
pub struct RequireRole {
    pub minimum_role: Role,
}

/// Helper that creates a `RequireRole` guard for Admin.
pub struct RequireAdmin;

impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // We expect AuthUser to have already been extracted and stored in
        // extensions by a prior layer or by extracting it here.
        let config = parts
            .extensions
            .get::<JwtConfig>()
            .cloned()
            .unwrap_or_default();

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| AppError::Unauthorized("Missing authentication token".into()))?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid authorization header".into()))?;

        let token = auth_str
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid authorization scheme".into()))?;

        let token_data = jwt::validate_token(token, &config)?;
        let role = jwt::role_from_claims(&token_data.claims)?;

        if !role.has_at_least(Role::Admin) {
            return Err(AppError::Forbidden(
                "Admin role required for this action".into(),
            ));
        }

        Ok(RequireAdmin)
    }
}

/// Helper that creates a `RequireRole` guard for Member (or above).
pub struct RequireMember;

impl<S> FromRequestParts<S> for RequireMember
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = parts
            .extensions
            .get::<JwtConfig>()
            .cloned()
            .unwrap_or_default();

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| AppError::Unauthorized("Missing authentication token".into()))?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid authorization header".into()))?;

        let token = auth_str
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid authorization scheme".into()))?;

        let token_data = jwt::validate_token(token, &config)?;
        let role = jwt::role_from_claims(&token_data.claims)?;

        if !role.has_at_least(Role::Member) {
            return Err(AppError::Forbidden(
                "Member role or above required for this action".into(),
            ));
        }

        Ok(RequireMember)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_hierarchy() {
        assert!(Role::Admin.has_at_least(Role::Admin));
        assert!(Role::Admin.has_at_least(Role::Member));
        assert!(Role::Admin.has_at_least(Role::Viewer));

        assert!(!Role::Member.has_at_least(Role::Admin));
        assert!(Role::Member.has_at_least(Role::Member));
        assert!(Role::Member.has_at_least(Role::Viewer));

        assert!(!Role::Viewer.has_at_least(Role::Admin));
        assert!(!Role::Viewer.has_at_least(Role::Member));
        assert!(Role::Viewer.has_at_least(Role::Viewer));
    }
}
