//! Part 6: Secrets Management
//!
//! Load secrets from environment variables, never log them, and scrub them
//! from error messages.

use std::fmt;

/// Application secrets loaded from environment variables.
///
/// TODO: Implement the following:
/// 1. `load()` — read from `APP_HMAC_KEY`, `APP_DATABASE_URL`, `APP_API_KEY`
/// 2. Custom `Debug` that prints `[REDACTED]` instead of actual values
/// 3. `scrub_secrets()` to remove secret values from strings
pub struct Secrets {
    pub hmac_key: String,
    pub database_url: String,
    pub api_key: String,
}

impl Secrets {
    /// Load secrets from environment variables.
    ///
    /// TODO: Read from `APP_HMAC_KEY`, `APP_DATABASE_URL`, and `APP_API_KEY`.
    /// Return an error if any are missing.
    pub fn load() -> Result<Self, SecretsError> {
        todo!("Load secrets from environment variables")
    }

    /// Create Secrets from explicit values (useful for testing).
    pub fn from_values(hmac_key: String, database_url: String, api_key: String) -> Self {
        Self {
            hmac_key,
            database_url,
            api_key,
        }
    }

    /// Scrub all secret values from the given string.
    ///
    /// TODO: Replace occurrences of each secret value with `[REDACTED]`.
    pub fn scrub_secrets(&self, _input: &str) -> String {
        todo!("Scrub secret values from the string")
    }
}

/// Custom Debug that never reveals actual secret values.
///
/// TODO: Print `Secrets { hmac_key: [REDACTED], database_url: [REDACTED], api_key: [REDACTED] }`
impl fmt::Debug for Secrets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implement redacted Debug output")
    }
}

/// Errors from secrets loading.
#[derive(Debug, Clone)]
pub enum SecretsError {
    Missing(String),
}

impl fmt::Display for SecretsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(var) => write!(f, "Missing environment variable: {var}"),
        }
    }
}

impl std::error::Error for SecretsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_does_not_reveal_secrets() {
        let secrets = Secrets::from_values(
            "super-secret-key".to_string(),
            "postgres://user:pass@localhost/db".to_string(),
            "api-key-12345".to_string(),
        );
        let debug_output = format!("{:?}", secrets);
        assert!(!debug_output.contains("super-secret-key"));
        assert!(!debug_output.contains("postgres://"));
        assert!(!debug_output.contains("api-key-12345"));
        assert!(debug_output.contains("REDACTED"));
    }

    #[test]
    fn test_scrub_secrets_from_error_message() {
        let secrets = Secrets::from_values(
            "my-hmac-key".to_string(),
            "postgres://admin:hunter2@db.example.com/prod".to_string(),
            "sk-live-abc123".to_string(),
        );
        let error_msg = "Connection failed: postgres://admin:hunter2@db.example.com/prod timed out";
        let scrubbed = secrets.scrub_secrets(error_msg);
        assert!(!scrubbed.contains("hunter2"));
        assert!(scrubbed.contains("[REDACTED]"));
    }

    #[test]
    fn test_scrub_preserves_non_secret_text() {
        let secrets = Secrets::from_values(
            "key".to_string(),
            "db-url".to_string(),
            "api-key".to_string(),
        );
        let msg = "Something went wrong with the request";
        let scrubbed = secrets.scrub_secrets(msg);
        assert_eq!(scrubbed, msg);
    }
}
