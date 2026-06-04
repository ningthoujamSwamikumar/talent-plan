/// Integration tests for authentication and authorization.
///
/// These tests validate the auth subsystem in isolation. Tests that require a
/// running database are marked with `#[ignore]` so that `cargo test` passes
/// without Postgres. Run ignored tests with `cargo test -- --ignored` after
/// setting up the database.
use taskforge_auth::auth::{api_key, jwt, password};
use taskforge_auth::models::Role;
use uuid::Uuid;
use chrono;

// ---------------------------------------------------------------------------
// Helper utilities
// ---------------------------------------------------------------------------

fn test_jwt_config() -> jwt::JwtConfig {
    jwt::JwtConfig {
        secret: "test-secret-key-for-unit-tests".to_string(),
        access_token_minutes: 15,
        refresh_token_days: 7,
    }
}

// ===========================================================================
// Registration tests
// ===========================================================================

#[test]
fn register_user_succeeds_with_valid_data() {
    // A valid CreateUser struct should pass validation.
    use validator::Validate;
    let user = taskforge_auth::models::CreateUser {
        email: "alice@example.com".to_string(),
        password: "strongPassword1!".to_string(),
        display_name: "Alice".to_string(),
    };
    assert!(user.validate().is_ok());
}

#[test]
fn register_user_fails_with_invalid_email() {
    use validator::Validate;
    let user = taskforge_auth::models::CreateUser {
        email: "not-an-email".to_string(),
        password: "strongPassword1!".to_string(),
        display_name: "Alice".to_string(),
    };
    assert!(user.validate().is_err());
}

#[test]
fn register_user_fails_with_weak_password() {
    use validator::Validate;
    let user = taskforge_auth::models::CreateUser {
        email: "alice@example.com".to_string(),
        password: "short".to_string(), // Less than 8 characters
        display_name: "Alice".to_string(),
    };
    let result = user.validate();
    assert!(result.is_err());
}

#[test]
#[ignore = "requires database"]
fn register_user_fails_with_duplicate_email() {
    // This test would register two users with the same email and assert
    // that the second attempt returns a Conflict error.
    todo!("Implement with database integration");
}

// ===========================================================================
// Login tests
// ===========================================================================

#[test]
fn login_succeeds_with_correct_credentials() {
    let raw_password = "correct_horse_battery_staple";
    let hash = password::hash_password(raw_password).unwrap();
    assert!(password::verify_password(raw_password, &hash).unwrap());
}

#[test]
fn login_fails_with_wrong_password() {
    let raw_password = "correct_horse_battery_staple";
    let hash = password::hash_password(raw_password).unwrap();
    assert!(!password::verify_password("wrong_password", &hash).unwrap());
}

#[test]
#[ignore = "requires database"]
fn login_fails_with_nonexistent_email() {
    // This test would attempt login with an email not in the database and
    // assert that InvalidCredentials is returned.
    todo!("Implement with database integration");
}

// ===========================================================================
// JWT validation tests
// ===========================================================================

#[test]
fn authenticated_request_succeeds_with_valid_jwt() {
    let config = test_jwt_config();
    let user_id = Uuid::new_v4();

    let token =
        jwt::create_access_token(user_id, "alice@example.com", &Role::Member, &config).unwrap();
    let decoded = jwt::validate_token(&token, &config).unwrap();

    assert_eq!(decoded.claims.sub, user_id.to_string());
    assert_eq!(decoded.claims.email, "alice@example.com");
    assert_eq!(decoded.claims.role, "member");
}

#[test]
fn authenticated_request_fails_without_token() {
    let config = test_jwt_config();
    let result = jwt::validate_token("", &config);
    assert!(result.is_err());
}

#[test]
fn authenticated_request_fails_with_expired_token() {
    // Manually craft a token with an expiry in the past.
    use jsonwebtoken::{encode, EncodingKey, Header};
    use taskforge_auth::auth::jwt::Claims;

    let config = test_jwt_config();
    let now = chrono::Utc::now();

    let claims = Claims {
        sub: Uuid::new_v4().to_string(),
        email: "alice@example.com".to_string(),
        role: "member".to_string(),
        iat: (now - chrono::Duration::hours(2)).timestamp(),
        exp: (now - chrono::Duration::hours(1)).timestamp(), // expired 1 hour ago
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .unwrap();

    let result = jwt::validate_token(&token, &config);
    assert!(result.is_err());
}

#[test]
fn authenticated_request_fails_with_invalid_token() {
    let config = test_jwt_config();
    let result = jwt::validate_token("not.a.valid.jwt.token", &config);
    assert!(result.is_err());
}

#[test]
fn authenticated_request_fails_with_wrong_secret() {
    let config = test_jwt_config();
    let user_id = Uuid::new_v4();

    let token =
        jwt::create_access_token(user_id, "alice@example.com", &Role::Member, &config).unwrap();

    let bad_config = jwt::JwtConfig {
        secret: "different-secret".to_string(),
        ..config
    };
    let result = jwt::validate_token(&token, &bad_config);
    assert!(result.is_err());
}

// ===========================================================================
// Role-based access control tests
// ===========================================================================

#[test]
fn admin_can_access_admin_only_routes() {
    assert!(Role::Admin.has_at_least(Role::Admin));
}

#[test]
fn member_cannot_access_admin_routes() {
    assert!(!Role::Member.has_at_least(Role::Admin));
}

#[test]
fn viewer_cannot_modify_resources() {
    // Viewers should not meet the Member threshold for write operations.
    assert!(!Role::Viewer.has_at_least(Role::Member));
}

#[test]
fn admin_has_all_permissions() {
    assert!(Role::Admin.has_at_least(Role::Admin));
    assert!(Role::Admin.has_at_least(Role::Member));
    assert!(Role::Admin.has_at_least(Role::Viewer));
}

#[test]
fn member_has_member_and_viewer_permissions() {
    assert!(Role::Member.has_at_least(Role::Member));
    assert!(Role::Member.has_at_least(Role::Viewer));
    assert!(!Role::Member.has_at_least(Role::Admin));
}

#[test]
fn role_claims_roundtrip_in_jwt() {
    let config = test_jwt_config();
    let user_id = Uuid::new_v4();

    for role in [Role::Viewer, Role::Member, Role::Admin] {
        let token =
            jwt::create_access_token(user_id, "test@example.com", &role, &config).unwrap();
        let decoded = jwt::validate_token(&token, &config).unwrap();
        let parsed_role = jwt::role_from_claims(&decoded.claims).unwrap();
        assert_eq!(parsed_role, role);
    }
}

// ===========================================================================
// Refresh token tests
// ===========================================================================

#[test]
fn refresh_token_is_unique() {
    let t1 = jwt::create_refresh_token();
    let t2 = jwt::create_refresh_token();
    assert_ne!(t1, t2);
}

#[test]
#[ignore = "requires database"]
fn refresh_token_returns_new_tokens() {
    // This test would:
    // 1. Login to get tokens
    // 2. Call POST /auth/refresh with the refresh token
    // 3. Assert new access_token and refresh_token are returned
    // 4. Assert old refresh token is invalidated
    todo!("Implement with database integration");
}

#[test]
#[ignore = "requires database"]
fn refresh_token_fails_with_invalid_token() {
    // This test would call POST /auth/refresh with a bogus token and assert
    // that an Unauthorized error is returned.
    todo!("Implement with database integration");
}

// ===========================================================================
// API key tests
// ===========================================================================

#[test]
fn api_key_generation_produces_valid_format() {
    let (key, prefix) = api_key::generate_api_key();
    assert!(key.starts_with("tfk_"));
    assert!(key.len() > 20);
    assert!(!prefix.is_empty());
}

#[test]
fn api_key_hash_and_verify() {
    let (key, _prefix) = api_key::generate_api_key();
    let hash = api_key::hash_api_key(&key);

    assert!(api_key::verify_api_key(&key, &hash));
}

#[test]
fn api_key_verify_rejects_wrong_key() {
    let (key, _) = api_key::generate_api_key();
    let hash = api_key::hash_api_key(&key);

    assert!(!api_key::verify_api_key("tfk_totally_wrong_key_value", &hash));
}

#[test]
#[ignore = "requires database"]
fn api_key_authentication_works() {
    // This test would:
    // 1. Create an API key via POST /auth/api-keys
    // 2. Make a request with X-API-Key header
    // 3. Assert the request is authenticated
    todo!("Implement with database integration");
}

#[test]
#[ignore = "requires database"]
fn api_key_can_be_revoked() {
    // This test would:
    // 1. Create an API key
    // 2. Revoke it via DELETE /auth/api-keys/:id
    // 3. Assert that subsequent requests with the key fail
    todo!("Implement with database integration");
}

// ===========================================================================
// Ownership tests
// ===========================================================================

#[test]
fn jwt_contains_user_id_for_ownership() {
    let config = test_jwt_config();
    let user_id = Uuid::new_v4();

    let token =
        jwt::create_access_token(user_id, "alice@example.com", &Role::Member, &config).unwrap();
    let decoded = jwt::validate_token(&token, &config).unwrap();
    let extracted_id = jwt::user_id_from_claims(&decoded.claims).unwrap();

    assert_eq!(extracted_id, user_id);
}

#[test]
#[ignore = "requires database"]
fn create_project_sets_owner_id_from_token() {
    // This test would:
    // 1. Register and login as a user
    // 2. Create a project via POST /projects with the JWT
    // 3. Assert that the project's owner_id matches the JWT's sub claim
    todo!("Implement with database integration");
}

#[test]
#[ignore = "requires database"]
fn user_can_only_update_own_project() {
    // This test would:
    // 1. User A creates a project
    // 2. User B tries to update it
    // 3. Assert 403 Forbidden
    todo!("Implement with database integration");
}

#[test]
#[ignore = "requires database"]
fn admin_can_update_any_project() {
    // This test would:
    // 1. User A (member) creates a project
    // 2. User B (admin) updates it
    // 3. Assert success
    todo!("Implement with database integration");
}

// ===========================================================================
// Security tests
// ===========================================================================

#[test]
fn password_is_never_returned_in_user_response() {
    use taskforge_auth::models::{User, UserResponse};

    let user = User {
        id: Uuid::new_v4(),
        email: "alice@example.com".to_string(),
        password_hash: "$argon2id$secret_hash_value".to_string(),
        display_name: "Alice".to_string(),
        role: Role::Member,
        failed_attempts: 0,
        locked_until: None,
        created_at: chrono::Utc::now(),
    };

    let response = UserResponse::from(user);
    let json = serde_json::to_string(&response).unwrap();

    assert!(!json.contains("password"));
    assert!(!json.contains("argon2"));
    assert!(!json.contains("hash"));
}

#[test]
fn password_is_hashed_not_plaintext() {
    let raw = "my_secret_password";
    let hash = password::hash_password(raw).unwrap();

    // The hash should not contain the plaintext password
    assert!(!hash.contains(raw));
    // The hash should be in PHC format (starts with $argon2)
    assert!(hash.starts_with("$argon2"));
}

#[test]
fn user_serialization_skips_password_hash() {
    let user = taskforge_auth::models::User {
        id: Uuid::new_v4(),
        email: "bob@example.com".to_string(),
        password_hash: "$argon2id$v=19$secret".to_string(),
        display_name: "Bob".to_string(),
        role: Role::Member,
        failed_attempts: 0,
        locked_until: None,
        created_at: chrono::Utc::now(),
    };

    let json = serde_json::to_string(&user).unwrap();
    // The password_hash field is marked #[serde(skip_serializing)]
    assert!(!json.contains("password_hash"));
    assert!(!json.contains("$argon2"));
}

// ===========================================================================
// Role parsing tests
// ===========================================================================

#[test]
fn role_roundtrip_string() {
    for role in [Role::Viewer, Role::Member, Role::Admin] {
        let s = role.to_string();
        let parsed: Role = s.parse().unwrap();
        assert_eq!(parsed, role);
    }
}

#[test]
fn role_parse_case_insensitive() {
    assert_eq!("ADMIN".parse::<Role>().unwrap(), Role::Admin);
    assert_eq!("Member".parse::<Role>().unwrap(), Role::Member);
    assert_eq!("viewer".parse::<Role>().unwrap(), Role::Viewer);
}

#[test]
fn role_parse_unknown_fails() {
    assert!("superuser".parse::<Role>().is_err());
}
