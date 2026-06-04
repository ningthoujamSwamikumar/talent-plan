use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ---------------------------------------------------------------------------
// Role enum
// ---------------------------------------------------------------------------

/// User roles ordered by increasing privilege: Viewer < Member < Admin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Viewer,
    Member,
    Admin,
}

impl Role {
    /// Returns a numeric privilege level for comparison.
    pub fn level(&self) -> u8 {
        match self {
            Role::Viewer => 0,
            Role::Member => 1,
            Role::Admin => 2,
        }
    }

    /// Returns true if this role meets or exceeds the required role.
    pub fn has_at_least(&self, required: Role) -> bool {
        self.level() >= required.level()
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Viewer => write!(f, "viewer"),
            Role::Member => write!(f, "member"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "viewer" => Ok(Role::Viewer),
            "member" => Ok(Role::Member),
            "admin" => Ok(Role::Admin),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

// ---------------------------------------------------------------------------
// User models
// ---------------------------------------------------------------------------

/// Full user record as stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    /// Never serialized in API responses — use `UserResponse` instead.
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: String,
    pub role: Role,
    pub failed_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Sanitized user returned in API responses (no password hash).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            role: user.role,
            created_at: user.created_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Auth request / response models
// ---------------------------------------------------------------------------

/// Registration request body.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(min = 1, max = 100, message = "Display name must be 1-100 characters"))]
    pub display_name: String,
}

/// Login request body.
#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Successful authentication response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

/// Refresh token request body.
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

// ---------------------------------------------------------------------------
// API key models
// ---------------------------------------------------------------------------

/// API key record as stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    /// Never returned after creation — use `ApiKeyResponse` for list views.
    #[serde(skip_serializing)]
    pub key_hash: String,
    pub key_prefix: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Response when creating a new API key (includes the plaintext key once).
#[derive(Debug, Clone, Serialize)]
pub struct ApiKeyCreatedResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub key_prefix: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// API key summary for listing (no secret material).
#[derive(Debug, Clone, Serialize)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(key: ApiKey) -> Self {
        ApiKeyResponse {
            id: key.id,
            name: key.name,
            key_prefix: key.key_prefix,
            expires_at: key.expires_at,
            revoked: key.revoked,
            last_used_at: key.last_used_at,
            created_at: key.created_at,
        }
    }
}

/// Request to create a new API key.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
    /// Optional expiry in days from now.
    pub expires_in_days: Option<i64>,
}

// ---------------------------------------------------------------------------
// Project and Task models (extended with owner_id)
// ---------------------------------------------------------------------------

/// A project owned by a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Request to create a new project.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateProject {
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: String,
    pub description: Option<String>,
}

/// Request to update a project.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateProject {
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: Option<String>,
    pub description: Option<String>,
}

/// A task within a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}
