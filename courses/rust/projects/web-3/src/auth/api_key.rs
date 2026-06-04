use rand::Rng;
use sha2::{Digest, Sha256};

use crate::error::AppError;

/// Prefix length for API keys (used for identification without exposing the key).
const KEY_PREFIX_LEN: usize = 8;

/// Full API key length in random bytes (before hex encoding).
const KEY_BYTES_LEN: usize = 32;

/// Generate a new random API key.
///
/// Returns `(plaintext_key, key_prefix)`. The caller must hash the plaintext
/// key before storing it in the database and return the plaintext to the user
/// exactly once.
pub fn generate_api_key() -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; KEY_BYTES_LEN];
    rng.fill(&mut bytes[..]);

    let key = format!("tfk_{}", hex::encode(&bytes));
    let prefix = key[..KEY_PREFIX_LEN.min(key.len())].to_string();

    (key, prefix)
}

/// Hash an API key using SHA-256 for storage.
///
/// We use SHA-256 rather than argon2 for API keys because API keys are
/// high-entropy random values (not human-chosen passwords), so a fast hash
/// is sufficient and allows quick lookup.
pub fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(&hasher.finalize())
}

/// Verify that a plaintext API key matches a stored hash.
pub fn verify_api_key(key: &str, stored_hash: &str) -> bool {
    let computed = hash_api_key(key);
    // Constant-time comparison would be ideal here; for this project the
    // simple equality check is acceptable since SHA-256 output is uniform.
    computed == stored_hash
}

/// Validate that a string looks like a valid API key format.
pub fn validate_api_key_format(key: &str) -> Result<(), AppError> {
    if !key.starts_with("tfk_") {
        return Err(AppError::Unauthorized("Invalid API key format".into()));
    }
    if key.len() < 20 {
        return Err(AppError::Unauthorized("Invalid API key format".into()));
    }
    Ok(())
}

// We need the hex crate for encoding. Since it's a very small dependency,
// we inline a minimal implementation here to avoid adding another crate.
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_api_key() {
        let (key, prefix) = generate_api_key();
        assert!(key.starts_with("tfk_"));
        assert_eq!(prefix.len(), KEY_PREFIX_LEN);
        assert!(key.len() > 20);
    }

    #[test]
    fn test_hash_and_verify() {
        let (key, _prefix) = generate_api_key();
        let hash = hash_api_key(&key);

        assert!(verify_api_key(&key, &hash));
        assert!(!verify_api_key("tfk_wrong_key", &hash));
    }

    #[test]
    fn test_validate_format() {
        assert!(validate_api_key_format("tfk_abcdef1234567890abcdef").is_ok());
        assert!(validate_api_key_format("bad_key").is_err());
        assert!(validate_api_key_format("tfk_short").is_err());
    }
}
