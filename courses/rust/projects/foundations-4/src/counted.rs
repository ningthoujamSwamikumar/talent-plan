use std::cell::Cell;
use std::ops::{Deref, DerefMut};

/// A smart pointer that counts how many times the inner value is accessed.
///
/// `Counted<T>` wraps a value of type `T` and keeps a running tally of how
/// many times the inner value has been accessed through `Deref` or `DerefMut`.
/// The count itself uses interior mutability (`Cell<usize>`) so that even
/// immutable dereferences are tracked.
///
/// # Examples
///
/// ```ignore
/// let c = Counted::new(String::from("hello"));
/// assert_eq!(c.access_count(), 0);
/// let _ = c.len();  // triggers Deref
/// assert_eq!(c.access_count(), 1);
/// ```
pub struct Counted<T> {
    // TODO: add fields
    //   - the inner value of type T
    //   - an access counter (hint: use Cell<usize> for interior mutability)
    _marker: std::marker::PhantomData<T>,
}

impl<T> Counted<T> {
    /// Creates a new `Counted` wrapping the given value with an access count of 0.
    pub fn new(value: T) -> Self {
        todo!()
    }

    /// Returns the number of times the inner value has been accessed via
    /// `Deref` or `DerefMut`.
    pub fn access_count(&self) -> usize {
        todo!()
    }

    /// Resets the access count to zero.
    pub fn reset_count(&self) {
        todo!()
    }
}

impl<T> Deref for Counted<T> {
    type Target = T;

    fn deref(&self) -> &T {
        todo!()
    }
}

impl<T> DerefMut for Counted<T> {
    fn deref_mut(&mut self) -> &mut T {
        todo!()
    }
}

impl<T> Drop for Counted<T> {
    fn drop(&mut self) {
        // Optional: students can log or track drops.
        // For example, print how many times the value was accessed before
        // being dropped.
    }
}
