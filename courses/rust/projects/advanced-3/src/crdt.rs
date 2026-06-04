use serde::{Deserialize, Serialize};

/// A Last-Writer-Wins Register.
///
/// Concurrent updates are resolved by timestamp; ties are broken by
/// lexicographic comparison of `writer_id` for deterministic convergence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwRegister<T: Clone> {
    pub value: T,
    pub timestamp: i64,
    pub writer_id: String,
}

impl<T: Clone> LwwRegister<T> {
    /// Create a new register with the given initial value. The timestamp is set
    /// to the current time in milliseconds.
    pub fn new(value: T, writer_id: String) -> Self {
        todo!()
    }

    /// Attempt to update the register. The update succeeds (returns `true`) if
    /// the provided timestamp is strictly greater than the current one, or if
    /// timestamps are equal and `writer_id` is lexicographically greater.
    pub fn update(&mut self, value: T, timestamp: i64, writer_id: String) -> bool {
        todo!()
    }

    /// Merge another register into this one. Returns `true` if this register's
    /// value changed.
    pub fn merge(&mut self, other: &LwwRegister<T>) -> bool {
        todo!()
    }
}
