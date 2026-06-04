//! Part 3: Security Headers
//!
//! Add standard security headers to every HTTP response.

use axum::{http::Request, middleware::Next, response::Response};

/// Middleware that adds security headers to every response.
///
/// TODO: Add the following headers:
/// - `Strict-Transport-Security: max-age=63072000; includeSubDomains`
/// - `Content-Security-Policy: default-src 'self'`
/// - `X-Content-Type-Options: nosniff`
/// - `X-Frame-Options: DENY`
/// - `X-XSS-Protection: 0`
/// - `Referrer-Policy: strict-origin-when-cross-origin`
pub async fn security_headers_middleware<B>(
    _request: Request<B>,
    _next: Next,
) -> Response {
    todo!("Add security headers to the response")
}

#[cfg(test)]
mod tests {
    // Security header tests are in tests/security_tests.rs since they require
    // a running server to inspect response headers.
}
