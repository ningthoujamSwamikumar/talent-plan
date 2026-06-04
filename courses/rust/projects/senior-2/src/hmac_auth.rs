//! Part 4: Request Signing (HMAC)
//!
//! Implement HMAC-SHA256 request signing for service-to-service authentication.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Sign a request and return the hex-encoded HMAC signature.
///
/// The signing string is: `"{method}\n{path}\n{timestamp}\n{body_hash}"`
/// where `body_hash` is the SHA-256 hex digest of the request body.
///
/// TODO: Implement HMAC-SHA256 signing.
///
/// # Arguments
/// * `key` - The shared HMAC key
/// * `method` - HTTP method (e.g., "POST")
/// * `path` - Request path (e.g., "/notes")
/// * `timestamp` - Unix timestamp as a string
/// * `body` - The raw request body bytes
pub fn sign_request(
    _key: &[u8],
    _method: &str,
    _path: &str,
    _timestamp: &str,
    _body: &[u8],
) -> String {
    todo!("Implement HMAC-SHA256 request signing")
}

/// Verify a request signature.
///
/// TODO: Recompute the signature and compare with the provided one.
/// Also check that the timestamp is within 5 minutes of the current time.
///
/// # Returns
/// - `Ok(())` if the signature is valid and the timestamp is fresh
/// - `Err(AuthError)` describing why verification failed
pub fn verify_request(
    _key: &[u8],
    _method: &str,
    _path: &str,
    _timestamp: &str,
    _body: &[u8],
    _provided_signature: &str,
) -> Result<(), AuthError> {
    todo!("Implement HMAC-SHA256 request verification")
}

/// Errors that can occur during request authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    /// The X-Signature header is missing.
    MissingSignature,
    /// The X-Timestamp header is missing.
    MissingTimestamp,
    /// The timestamp is too old (replay protection).
    ExpiredTimestamp,
    /// The signature does not match.
    InvalidSignature,
    /// The timestamp is not a valid number.
    InvalidTimestamp,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSignature => write!(f, "Missing X-Signature header"),
            Self::MissingTimestamp => write!(f, "Missing X-Timestamp header"),
            Self::ExpiredTimestamp => write!(f, "Timestamp expired (replay protection)"),
            Self::InvalidSignature => write!(f, "Invalid signature"),
            Self::InvalidTimestamp => write!(f, "Invalid timestamp format"),
        }
    }
}

impl std::error::Error for AuthError {}

/// Axum middleware that verifies HMAC signatures on incoming requests.
///
/// TODO: Extract X-Signature and X-Timestamp headers, read the body,
/// call `verify_request`, and return 401 if verification fails.
///
/// This middleware should only be applied to routes that require authentication.
pub async fn hmac_auth_middleware() {
    todo!("Implement HMAC auth middleware for axum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_roundtrip() {
        let key = b"test-secret-key";
        let method = "POST";
        let path = "/notes";
        let timestamp = "1700000000";
        let body = b"hello world";

        let signature = sign_request(key, method, path, timestamp, body);
        let result = verify_request(key, method, path, timestamp, body, &signature);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tampered_body_fails_verification() {
        let key = b"test-secret-key";
        let method = "POST";
        let path = "/notes";
        let timestamp = "1700000000";

        let signature = sign_request(key, method, path, timestamp, b"original body");
        let result = verify_request(key, method, path, timestamp, b"tampered body", &signature);
        assert_eq!(result, Err(AuthError::InvalidSignature));
    }

    #[test]
    fn test_wrong_key_fails_verification() {
        let method = "POST";
        let path = "/notes";
        let timestamp = "1700000000";
        let body = b"hello";

        let signature = sign_request(b"key-a", method, path, timestamp, body);
        let result = verify_request(b"key-b", method, path, timestamp, body, &signature);
        assert_eq!(result, Err(AuthError::InvalidSignature));
    }
}
