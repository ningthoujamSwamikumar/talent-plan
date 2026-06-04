# Building Block Web-3: Authentication and Security

**Prerequisites**: [Project: Database Layer](../projects/web-2/README.md).

Before starting [Project: Auth & Authorization](../projects/web-3/README.md), complete
the readings and exercises below.

## What to read

- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html).
  The definitive guide to implementing authentication securely.

- [JWT Introduction](https://jwt.io/introduction).
  Understand the three parts of a JWT (header, payload, signature), how signing
  works, and why JWTs are stateless.

- [JSON Web Token Best Practices (RFC 8725)](https://datatracker.ietf.org/doc/html/rfc8725).
  Common pitfalls and how to avoid them. Important: always validate the `alg` header,
  set short expiration times, use strong signing keys.

- [Argon2 password hashing](https://docs.rs/argon2/latest/argon2/).
  Why Argon2 is recommended over bcrypt/scrypt. How to configure memory, iterations,
  and parallelism parameters.

- [jsonwebtoken crate documentation](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/).
  Rust JWT library. Focus on `encode()`, `decode()`, and `Validation`.

## Key concepts

### Authentication vs Authorization

- **Authentication (AuthN)**: Who are you? Verifying identity via credentials.
- **Authorization (AuthZ)**: What can you do? Checking permissions.

### JWT Flow

```
1. Client sends credentials (POST /auth/login)
2. Server verifies credentials, creates JWT with user claims
3. Server returns JWT to client
4. Client includes JWT in subsequent requests (Authorization: Bearer <token>)
5. Server validates JWT signature and expiration on each request
6. Server extracts user info from claims to authorize actions
```

### Password Hashing

```rust
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};

// Hashing (during registration)
let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default()
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

// Verifying (during login)
let parsed_hash = PasswordHash::new(&stored_hash)?;
Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
```

### Role-Based Access Control (RBAC)

```
Admin   → full access to everything
Member  → CRUD on own resources, read others
Viewer  → read-only access
```

Implement as middleware that checks the role claim in the JWT.

## Exercises

**Exercise 1**: Implement JWT encode/decode in a scratch project:
```rust
let token = jsonwebtoken::encode(&header, &claims, &encoding_key)?;
let decoded = jsonwebtoken::decode::<Claims>(&token, &decoding_key, &validation)?;
```

**Exercise 2**: Hash a password with argon2 and verify it. Time the hashing —
it should take 100ms+ (that's intentional, for security).

**Exercise 3**: Create an axum middleware that extracts and validates a JWT from
the `Authorization` header. Return 401 if missing or invalid.

**Exercise 4**: Implement a `RequireRole` extractor that wraps the auth extractor
and additionally checks the user's role.

## You're ready when...

- [ ] You can explain the JWT flow (issue, send, validate)
- [ ] You can hash and verify passwords with argon2
- [ ] You understand the difference between authentication and authorization
- [ ] You can implement middleware-based auth in axum
- [ ] You know why you should never store passwords in plaintext

Next: [Project: Auth & Authorization](../projects/web-3/README.md)
