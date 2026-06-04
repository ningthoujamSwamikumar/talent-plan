# Phase 6, Project 2: Security Hardening

In this project you will harden a web API against common security threats. You will
sanitize user input to prevent XSS, guard against SQL injection, add standard security
headers, implement HMAC-based request signing for service-to-service authentication,
build structured audit logging, manage secrets safely, and integrate dependency auditing
into your workflow.

These are the security practices every production Rust backend should implement.

## Prerequisites

- Completion of Phase 5 and senior-1
- Familiarity with axum middleware and extractors
- Basic understanding of cryptographic hashing concepts

## Project Overview

You are hardening a simple note-taking API. Notes have an `id`, `title`, `content` (which
may contain user-supplied HTML), `author`, `created_at`, and `updated_at`. The API supports
CRUD operations, and you will layer security on top of the functional implementation.

## Part 1: Input Sanitization

**Goal:** Strip potentially malicious HTML/JS from user-supplied content before storing it.

**Tasks:**
1. Implement `sanitize_html` in `src/sanitize.rs` using the `ammonia` crate
2. Allow only safe tags: `<b>`, `<i>`, `<em>`, `<strong>`, `<p>`, `<br>`, `<ul>`, `<ol>`, `<li>`
3. Strip all attributes except `href` on `<a>` tags (and ensure `href` only allows `http`/`https`)
4. Apply sanitization to `title` and `content` fields on every create/update
5. Write tests proving that `<script>`, `onclick`, `javascript:` URIs, and `<iframe>` are removed

**Hints:**
- `ammonia::Builder::new().tags(...)` lets you whitelist specific tags
- The `clean()` method returns a sanitized string
- Test with adversarial inputs: nested tags, encoded entities, etc.

## Part 2: SQL Injection Prevention

**Goal:** Demonstrate that parameterized queries protect against injection, and that string
concatenation does not.

**Tasks:**
1. In `src/lib.rs`, implement a `NoteStore` trait with `find_by_title(title: &str)` method
2. Implement `SafeNoteStore` that uses parameterized queries (simulated with a HashMap and
   exact-match lookup)
3. Implement `UnsafeNoteStore` that concatenates the title into a "query string" to demonstrate
   the vulnerability
4. Write tests showing that `UnsafeNoteStore` is vulnerable to injection-like input while
   `SafeNoteStore` is not

**Note:** Since this project uses in-memory storage (no real SQL), the "injection" is simulated.
The point is understanding WHY parameterized queries matter.

## Part 3: Security Headers

**Goal:** Add standard security headers to every HTTP response.

**Tasks:**
1. Implement `security_headers_layer` in `src/security_headers.rs`
2. Add these headers:
   - `Strict-Transport-Security: max-age=63072000; includeSubDomains` (HSTS)
   - `Content-Security-Policy: default-src 'self'` (CSP)
   - `X-Content-Type-Options: nosniff`
   - `X-Frame-Options: DENY`
   - `X-XSS-Protection: 0` (modern browsers should use CSP instead)
   - `Referrer-Policy: strict-origin-when-cross-origin`
3. Apply the layer to the entire application router
4. Write tests verifying each header is present and has the correct value

**Hints:**
- `tower_http::set_header::SetResponseHeaderLayer` can add individual headers
- Alternatively, use `axum::middleware::from_fn` to add all headers in one middleware

## Part 4: Request Signing (HMAC)

**Goal:** Implement HMAC-SHA256 request signing for service-to-service authentication.

**Tasks:**
1. Implement `sign_request` and `verify_request` in `src/hmac_auth.rs`
2. The signature covers: HTTP method + path + timestamp + body hash
3. The signature is sent in the `X-Signature` header; the timestamp in `X-Timestamp`
4. Reject requests where the timestamp is more than 5 minutes old (replay protection)
5. Reject requests where the signature does not match
6. Create an axum middleware that verifies the signature on protected routes
7. Write tests for valid signatures, tampered bodies, expired timestamps, and missing headers

**Hints:**
- Use `hmac::Hmac<sha2::Sha256>` for the HMAC computation
- The signing string should be deterministic: `"{method}\n{path}\n{timestamp}\n{body_hash}"`
- `hex::encode` the final MAC for the header value (or use base64)

## Part 5: Audit Logging

**Goal:** Log WHO did WHAT to WHICH resource WHEN from WHERE.

**Tasks:**
1. Define an `AuditEvent` struct in `src/audit.rs` with fields:
   - `who`: user/service identifier
   - `action`: what they did (create, read, update, delete)
   - `resource_type`: type of resource (e.g., "note")
   - `resource_id`: ID of the resource
   - `when`: timestamp
   - `ip_address`: client IP
   - `details`: optional additional context
2. Implement an `AuditLogger` trait and an `InMemoryAuditLogger` for testing
3. Create an axum middleware that logs every request as an audit event
4. Write tests verifying that CRUD operations produce the correct audit events

**Hints:**
- Use `tracing::info!` for production logging, but keep an in-memory buffer for tests
- The middleware can extract the client IP from `ConnectInfo` or `X-Forwarded-For`
- Store the audit log in shared state (`Arc<RwLock<Vec<AuditEvent>>>`) for testing

## Part 6: Secrets Management

**Goal:** Load secrets from environment variables, never log them, and scrub them from
error messages.

**Tasks:**
1. Implement a `Secrets` struct in `src/secrets.rs` that loads from env vars:
   - `APP_HMAC_KEY` — the HMAC signing key
   - `APP_DATABASE_URL` — simulated database connection string
   - `APP_API_KEY` — an API key for external services
2. Implement `Debug` manually to redact secret values (print `[REDACTED]` instead)
3. Implement a `scrub_secrets` function that removes secret values from error message strings
4. Verify that `format!("{:?}", secrets)` never reveals actual values
5. Verify that error messages passed through `scrub_secrets` have secrets removed

**Hints:**
- Use `std::env::var("APP_HMAC_KEY")` to load
- Custom `Debug`: `impl fmt::Debug for Secrets { fn fmt(...) { write!(f, "Secrets {{ [REDACTED] }}") } }`
- `scrub_secrets` can use `str::replace` for each known secret value

## Part 7: Dependency Auditing

**Goal:** Understand how to audit your dependency tree for known vulnerabilities.

**Tasks:**
1. Document how to install `cargo-audit`: `cargo install cargo-audit`
2. Run `cargo audit` on this project and document the results
3. Create a `deny.toml` (for `cargo-deny`) configuration that:
   - Bans specific problematic crates
   - Sets license requirements (e.g., only allow MIT, Apache-2.0, BSD)
   - Configures advisory checking
4. This part is primarily documentation — add instructions to this README

To run an audit:
```bash
cargo install cargo-audit
cargo audit
```

To use cargo-deny:
```bash
cargo install cargo-deny
cargo deny init
cargo deny check
```

## Testing

Run all tests:
```
cargo test
```

The test suite in `tests/security_tests.rs` covers all parts above.

## What You Will Learn

- Input sanitization to prevent XSS attacks
- Why parameterized queries prevent SQL injection
- Standard security headers and what each one protects against
- HMAC-based authentication for service-to-service communication
- Structured audit logging for compliance and debugging
- Secrets management best practices
- Dependency auditing with cargo-audit and cargo-deny
