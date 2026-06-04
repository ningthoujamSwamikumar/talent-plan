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

## You're ready when...

- [ ] You can name the OWASP Top 10
- [ ] You know how to prevent SQL injection and XSS
- [ ] You understand TLS configuration basics
- [ ] You can implement audit logging for sensitive operations

Next: [Project: Security Hardening](../projects/senior-2/README.md)
