//! Part 3: Code Review Exercise
//!
//! THIS FILE CONTAINS INTENTIONAL BUGS, SECURITY ISSUES, AND STYLE PROBLEMS.
//! Do NOT fix this file -- review it and write your findings.
//!
//! Pretend this is a pull request for a user authentication module.
//! Your job: find as many issues as you can across these categories:
//! - Bugs (logic errors, off-by-one, incorrect error handling)
//! - Security (plaintext passwords, timing attacks, missing validation)
//! - Performance (unnecessary cloning, O(n) lookups)
//! - Style (naming, documentation, dead code)
//! - Rust-specific (unwrap in production, unused Results, Send/Sync)
//!
//! There are approximately 15 issues. See `review_answers.md` for the answer key.

#![allow(dead_code, unused_variables, unused_must_use)]

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

/// Global user counter -- not thread-safe!
static mut USER_COUNTER: u64 = 0;

// Issue: no documentation on public struct
/// User authentication service.
pub struct UserAuthService {
    // Issue: Mutex<HashMap> without Arc -- not shareable across threads
    users: Mutex<HashMap<String, UserRecord>>,
    // Issue: hardcoded secret in source code
    jwt_secret: String,
    log_file: String,
}

struct UserRecord {
    username: String,
    // BUG: password stored in plaintext -- should be hashed
    password: String,
    email: String,
    // Issue: field name is vague
    flag: bool,
    login_count: i32, // Issue: should be u32, login count cannot be negative
}

impl UserAuthService {
    /// Creates a new authentication service.
    pub fn new(log_file: &str) -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            // SECURITY: hardcoded secret
            jwt_secret: "super-secret-key-123".to_string(),
            log_file: log_file.to_string(),
        }
    }

    // Issue: missing documentation
    // BUG: does not validate email format
    // BUG: does not check if username already exists
    pub fn register(&self, username: String, password: String, email: String) -> bool {
        // Issue: no input validation (empty username, password length, etc.)
        let mut users = self.users.lock().unwrap(); // Issue: unwrap on Mutex lock

        // ISSUE: SQL injection -- building query via format!
        let _query = format!(
            "INSERT INTO users (username, password, email) VALUES ('{}', '{}', '{}')",
            username, password, email
        );

        // ISSUE: Race condition -- unsafe mutable static
        let id = unsafe {
            USER_COUNTER += 1;
            USER_COUNTER
        };

        let record = UserRecord {
            username: username.clone(), // Issue: unnecessary clone, username is already owned
            password,                   // SECURITY: plaintext password storage
            email,
            flag: true,
            login_count: 0,
        };

        // ISSUE: Password logged in plaintext
        self.log(&format!(
            "New user registered: {} with password {}",
            &record.username, &record.password
        ));

        users.insert(username, record);
        true // BUG: always returns true even if user already existed
    }

    // SECURITY: timing attack vulnerability -- string comparison is not constant-time
    pub fn authenticate(&self, username: &str, password: &str) -> Result<String, String> {
        let users = self.users.lock().unwrap();

        // BUG: O(n) iteration when we have a HashMap -- should use .get()
        for (key, record) in users.iter() {
            if key == username {
                // SECURITY: direct string comparison vulnerable to timing attacks
                if record.password == password {
                    record.login_count; // BUG: doesn't actually increment (not mutable)
                    return Ok(format!("token-{}", username)); // Issue: not a real JWT
                } else {
                    return Err("wrong password".to_string()); // Issue: reveals that user exists
                }
            }
        }
        Err("user not found".to_string()) // Issue: different error reveals user doesn't exist
    }

    // BUG: off-by-one in the length check, logic is inverted
    pub fn validate_password(&self, password: &str) -> bool {
        // BUG: should be >= 8, but uses > 8 (requires 9 chars minimum)
        if password.len() > 8 {
            return false; // BUG: logic inverted -- returns false when password IS long enough
        }
        true
    }

    // UNUSED: this function is never called
    fn _unused_helper(&self) -> Vec<String> {
        let users = self.users.lock().unwrap();
        let mut result = Vec::new();
        for (username, _) in users.iter() {
            result.push(username.clone());
        }
        result // Issue: could just use .keys().cloned().collect()
    }

    /// Deletes a user by username.
    pub fn delete_user(&self, username: &str) {
        let mut users = self.users.lock().unwrap();
        users.remove(username);
        // Issue: no return value indicating whether the user existed
        // Issue: no audit logging of user deletion
    }

    // Issue: this function silently ignores errors
    pub fn update_email(&self, username: &str, new_email: &str) {
        let mut users = self.users.lock().unwrap();
        if let Some(record) = users.get_mut(username) {
            record.email = new_email.to_string();
        }
        // Issue: no indication of whether the user was found
        // Issue: no email validation
    }

    /// Writes a log message to the log file.
    ///
    /// BUG: Missing error handling on file I/O.
    fn log(&self, message: &str) {
        // ISSUE: Missing error handling -- ignores Result from file operations
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file);

        if let Ok(mut f) = file {
            f.write_all(message.as_bytes());
            f.write_all(b"\n");
        }
    }
}
