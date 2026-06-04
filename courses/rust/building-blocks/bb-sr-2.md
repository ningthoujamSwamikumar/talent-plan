# Building Block Sr-2: Security Hardening

**Prerequisites**: [Project: API Design & Versioning](../projects/senior-1/README.md).

Before starting [Project: Security Hardening](../projects/senior-2/README.md),
complete the readings below.

## What to read

- [OWASP Top 10](https://owasp.org/www-project-top-ten/).
  The ten most critical web application security risks. Every backend engineer
  must know these.

- [OWASP Rust Security Guidelines](https://cheatsheetseries.owasp.org/cheatsheets/Rust_Security_Cheat_Sheet.html).
  Rust-specific security considerations.

- [cargo-audit](https://rustsec.org/).
  Audit your dependencies for known vulnerabilities. Run regularly.

- [RustLS documentation](https://docs.rs/rustls/latest/rustls/).
  Pure-Rust TLS implementation. Safer than OpenSSL bindings.

- [Secure Coding Practices Quick Reference](https://owasp.org/www-pdf-archive/OWASP_SCP_Quick_Reference_Guide_v2.pdf).
  Practical checklist for secure development.

## Key concepts

### Common Vulnerabilities in Web APIs

| Vulnerability | Prevention |
|--------------|------------|
| SQL Injection | Parameterized queries (sqlx does this) |
| XSS | Sanitize stored HTML with ammonia |
| CSRF | SameSite cookies, CSRF tokens |
| Insecure Direct Object Reference | Check ownership, not just existence |
| Secrets in Logs | Never log passwords, tokens, API keys |
| Dependency Vulnerabilities | Regular cargo-audit runs |

### Security Headers

```
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
```

### Audit Logging
Log WHO did WHAT to WHICH resource WHEN from WHERE:
```json
{"user_id":"abc","action":"delete_task","resource_id":"xyz","timestamp":"...","ip":"..."}
```

## Exercises

1. **Find the vulnerabilities.** Review this code and list every security issue:
   ```rust
   async fn create_user(Json(body): Json<CreateUser>) -> impl IntoResponse {
       let query = format!("INSERT INTO users (name, email) VALUES ('{}', '{}')", body.name, body.email);
       let password_hash = format!("{:x}", md5::compute(&body.password));
       let token = format!("user_{}", body.email);
       tracing::info!("Created user with password {}", body.password);
       // ...
   }
   ```

2. **Threat model an API.** Take your capstone project's API. For each
   endpoint, answer: What could an attacker do if they had access to this
   endpoint? What authentication/authorization is required? What input
   validation is needed? What rate limiting should apply?

3. **Implement a timing-safe comparison.** Explain why `==` for comparing
   password hashes is vulnerable to timing attacks. Write a constant-time
   comparison function using `subtle::ConstantTimeEq` or manual
   XOR-and-accumulate. Benchmark both to show the timing difference.

4. **Audit a `Cargo.lock`.** Run `cargo audit` on one of your course projects.
   If no vulnerabilities are found, look at the dependency tree with
   `cargo tree` and identify: which dependencies are largest? Which have the
   most transitive dependencies? Which haven't been updated in over a year?

## You're ready when...

- [ ] You can name the OWASP Top 10 and describe mitigations for each
- [ ] You know how to prevent SQL injection, XSS, and CSRF
- [ ] You understand timing attacks and constant-time comparison
- [ ] You can implement audit logging for sensitive operations
- [ ] You can audit a dependency tree for security risks

Next: [Project: Security Hardening](../projects/senior-2/README.md)
