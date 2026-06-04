# Code Review Answer Key: `src/review_exercise.rs`

This document lists all intentional issues in the code review exercise file.

## Security Issues

1. **Plaintext password storage** (register, UserRecord) -- Passwords are stored as plain
   strings. They should be hashed with a strong algorithm like argon2 or bcrypt before
   storage.

2. **Hardcoded JWT secret** (new) -- The JWT signing secret is a hardcoded string literal.
   It should be loaded from an environment variable or a secrets manager.

3. **SQL injection vulnerability** (register) -- The SQL query is built via `format!` with
   user-supplied values concatenated directly. Use parameterized queries instead.

4. **Password logged in plaintext** (register) -- The log call includes the raw password,
   which means passwords end up in log files in cleartext.

5. **Timing attack in authenticate** -- The `==` comparison on passwords leaks information
   about the password through timing side-channels. Use a constant-time comparison function.

6. **Error messages reveal user existence** (authenticate) -- Different error messages for
   "user not found" vs "wrong password" allow an attacker to enumerate valid usernames.
   Use a single generic "invalid credentials" message.

## Bugs

7. **Race condition on `USER_COUNTER`** (register) -- A `static mut` is modified inside
   `unsafe` without synchronization. Multiple threads calling `register` concurrently will
   cause undefined behavior. Use `AtomicU64` instead.

8. **`register` always returns true** -- The function does not check whether the username
   already exists in the map. `HashMap::insert` silently overwrites, so duplicate
   registrations succeed without notice.

9. **`validate_password` logic is inverted** -- The function returns `false` when the
   password length is greater than 8, which is the opposite of the intended check (should
   reject short passwords, not long ones).

10. **`validate_password` off-by-one** -- Uses `> 8` instead of `>= 8`, effectively
    requiring 9 characters minimum instead of 8.

11. **`login_count` is never incremented** (authenticate) -- The expression
    `record.login_count;` is a no-op. It reads the value but does not assign anything.
    It would need to be `record.login_count += 1;` through a mutable reference.

12. **`authenticate` uses O(n) iteration on a HashMap** -- The code iterates over all
    entries to find a username, when `HashMap::get()` provides O(1) lookup.

## Performance / Style

13. **Unnecessary clone of `username`** (register) -- `username` is an owned `String` that
    is moved into the HashMap key, but `username.clone()` is used for the `UserRecord` field
    unnecessarily.

14. **Vague field name `flag`** -- The `bool` field `flag` in `UserRecord` has no
    descriptive name. It should be something like `is_active` or `is_verified`.

15. **`login_count` is `i32`** -- A login count cannot be negative. It should be `u32` or
    `u64`.

16. **`_unused_helper` uses manual loop** -- The function could be written as
    `users.keys().cloned().collect()` instead of a manual loop with `push`.

## Rust-Specific

17. **`unwrap()` on `Mutex::lock()`** -- If another thread panics while holding the lock,
    the Mutex becomes poisoned and `unwrap()` will panic. Production code should handle
    `PoisonError`.

18. **Missing error handling on file I/O** (log) -- The `write_all` calls return `Result`
    values that are silently ignored. At minimum, errors should be logged to stderr.

19. **`update_email` and `delete_user` silently ignore missing users** -- Neither function
    indicates whether the operation actually found and modified/removed a user. They should
    return `bool` or `Result`.

20. **Missing documentation on public methods** -- Most public methods lack doc comments,
    violating Rust documentation conventions.
