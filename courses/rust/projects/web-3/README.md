# Project: Authentication and Authorization

## Phase 2, Project 3 — TaskForge Auth

### Introduction

Security is not optional. Every real-world API must answer two fundamental questions
for every incoming request:

1. **Authentication** — *Who are you?* Proving identity through credentials.
2. **Authorization** — *What can you do?* Determining which resources and actions
   the authenticated identity is permitted to access.

This project adds both layers to the task management API you have been building.
You will implement JSON Web Tokens (JWT) for stateless authentication, argon2
password hashing for secure credential storage, and role-based access control
(RBAC) so that different users have different permissions.

#### Common Authentication Strategies

| Strategy | How It Works | Trade-offs |
|---|---|---|
| **Session tokens** | Server stores session state; client sends a cookie. | Simple, but requires server-side storage and does not scale horizontally without shared state. |
| **JWT** | Server signs a self-contained token; client sends it in the `Authorization` header. | Stateless and scalable, but tokens cannot be individually revoked without extra infrastructure. |
| **API keys** | A long-lived secret string sent in a header. | Easy for machine-to-machine, but risky if leaked; no built-in expiry. |

In this project you will implement JWT as the primary mechanism, with API keys as
an alternative for programmatic access.

---

### Part 1: User Model

Add a `users` table to the database:

```sql
CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email       VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    role        VARCHAR(20) NOT NULL DEFAULT 'member',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Role Enum

Define three roles with increasing privilege:

- **Viewer** — read-only access to projects and tasks.
- **Member** — full CRUD on resources the user owns.
- **Admin** — full access to all resources, plus user management.

#### Password Hashing with Argon2

Never store plaintext passwords. The `argon2` crate provides the current
best-practice algorithm for password hashing. Key points:

- `argon2::hash_encoded` produces a string that embeds the salt, algorithm
  parameters, and the hash itself.
- `argon2::verify_encoded` checks a candidate password against a stored hash.
- Use a cryptographically secure random salt for each password (the `rand` crate).

**Exercise:** Implement `hash_password(raw: &str) -> Result<String>` and
`verify_password(raw: &str, hash: &str) -> Result<bool>` in `src/auth/password.rs`.

---

### Part 2: Registration and Login

#### POST /auth/register

1. Validate the request body (email format, password strength, display name
   length).
2. Check that no user with the same email exists.
3. Hash the password.
4. Insert the new user and return a sanitized user object (no password hash).

#### POST /auth/login

1. Look up the user by email.
2. Verify the candidate password against the stored hash.
3. On success, generate an access token (JWT) and a refresh token.
4. Return both tokens plus basic user info.

#### JWT Structure

A JWT has three Base64url-encoded parts separated by dots:

```
header.payload.signature
```

- **Header** — `{"alg": "HS256", "typ": "JWT"}`
- **Payload** — your custom claims: `sub` (user id), `email`, `role`, `exp`
  (expiry), `iat` (issued at).
- **Signature** — `HMAC-SHA256(base64(header) + "." + base64(payload), secret)`.

The `jsonwebtoken` crate handles encoding and decoding. You supply a `Claims`
struct and a secret key.

**Exercise:** Implement the register and login handlers in
`src/handlers/auth.rs`. Write the JWT helpers in `src/auth/jwt.rs`.

---

### Part 3: JWT Middleware

Create an axum extractor that:

1. Reads the `Authorization` header from the request.
2. Strips the `Bearer ` prefix.
3. Decodes and validates the JWT using the shared secret.
4. Rejects the request with `401 Unauthorized` if the token is missing, invalid,
   or expired.
5. On success, makes the authenticated user's claims available to the handler.

```rust
/// Extractor that validates JWT and provides authenticated user info.
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: Role,
}
```

Axum's `FromRequestParts` trait lets you build custom extractors that run before
the handler body. If extraction fails, the handler is never called.

**Exercise:** Implement `AuthUser` in `src/auth/middleware.rs`.

---

### Part 4: Protected Routes

Wrap your existing CRUD routes so they require authentication:

```rust
// Before (public):
Router::new().route("/projects", post(create_project))

// After (protected):
Router::new().route("/projects", post(create_project))
// create_project now takes AuthUser as a parameter
```

Key changes:

- `create_project` and `create_task` set `owner_id` from the token's `sub`
  claim.
- `update_project` and `delete_project` verify that `owner_id == auth_user.user_id`
  (unless the user is an admin).
- `list_projects` returns only projects owned by the current user (admins see
  all).

**Exercise:** Update every handler to accept `AuthUser` and enforce ownership
rules.

---

### Part 5: Role-Based Access Control

Implement a generic role guard extractor:

```rust
pub struct RequireRole<const R: u8>;
```

Or, more readably with an enum approach:

```rust
pub struct RequireRole {
    pub minimum_role: Role,
}
```

Roles are ordered: `Viewer < Member < Admin`. A `RequireRole(Admin)` extractor
rejects any user whose role is below Admin with `403 Forbidden`.

| Role | Permissions |
|---|---|
| **Admin** | Full access to all resources. Can manage users. |
| **Member** | Create, read, update, delete own projects and tasks. |
| **Viewer** | Read-only access to projects and tasks they can view. |

**Exercise:** Implement `RequireRole` as an axum extractor in
`src/auth/middleware.rs`. Apply it to admin-only routes (e.g., listing all users).

---

### Part 6: Refresh Tokens

Access tokens should be short-lived (e.g., 15 minutes). Refresh tokens are
longer-lived (e.g., 7 days) and allow the client to obtain new access tokens
without re-entering credentials.

#### Flow

1. Login returns `access_token` (short-lived) + `refresh_token` (long-lived).
2. Client stores both tokens.
3. When the access token expires, the client calls `POST /auth/refresh` with the
   refresh token.
4. Server validates the refresh token, issues a new access + refresh token pair,
   and invalidates the old refresh token (**token rotation**).

#### Token Rotation

Store refresh tokens (hashed) in the database. When a refresh token is used:

1. Look up the hashed token in the database.
2. Verify it has not been revoked or expired.
3. Delete the old token, insert a new one.
4. Return the new token pair.

This limits the damage if a refresh token is stolen — the legitimate user's next
refresh attempt will fail, alerting them.

**Exercise:** Implement `POST /auth/refresh` and the database operations in
`src/repository/users.rs`.

---

### Part 7: API Key Authentication

Some clients (CI/CD pipelines, scripts) prefer API keys over JWT. Support an
alternative authentication path:

- Users create API keys via `POST /auth/api-keys` (requires JWT auth).
- The server generates a random key, stores its hash (argon2 or SHA-256), and
  returns the plaintext key **once**.
- Subsequent requests can authenticate with `X-API-Key: <key>` instead of
  `Authorization: Bearer <token>`.
- Users can list and revoke their API keys.

**Exercise:** Implement API key endpoints and update the auth middleware to check
for `X-API-Key` as a fallback when no Bearer token is present.

---

### Part 8: Security Best Practices

#### Rate Limiting

Use `tower::limit::RateLimitLayer` or a custom middleware to throttle login
attempts (e.g., 5 attempts per minute per IP).

#### Account Lockout

After N consecutive failed login attempts, temporarily lock the account:

```sql
ALTER TABLE users ADD COLUMN failed_attempts INT DEFAULT 0;
ALTER TABLE users ADD COLUMN locked_until TIMESTAMPTZ;
```

#### Secure Token Storage (Client Guidance)

- **Access tokens:** In-memory only. Never in localStorage.
- **Refresh tokens:** HttpOnly, Secure, SameSite=Strict cookie (for web apps).
- **API keys:** Environment variables or secret managers. Never in source code.

#### Never Log Secrets

Ensure your tracing/logging configuration never prints passwords, tokens, or API
keys. Use `#[serde(skip_serializing)]` on sensitive fields and redact in logs.

#### HTTPS

Always serve production APIs over HTTPS. TLS termination is typically handled by
a reverse proxy (nginx, Cloudflare) rather than the Rust application itself.

---

### Summary

In this project you learned:

- How to hash passwords securely with argon2.
- How JWTs work and how to issue/validate them.
- How to build axum extractors for authentication and authorization.
- How to enforce ownership and role-based access control.
- How refresh tokens and token rotation improve security.
- How API keys offer an alternative authentication mechanism.
- Security best practices for production APIs.

### Next Steps

In the next project, you will add database migrations and advanced query patterns
to TaskForge, including pagination, filtering, full-text search, and connection
pooling.
