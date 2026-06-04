#![deny(missing_docs)]
//! System design implementations and interview preparation.
//!
//! This crate contains implementations of classic system design problems
//! and a code review exercise for interview preparation.
//!
//! # Interview Questions (Part 4)
//!
//! These model answers are provided as documentation. Students should attempt
//! to answer each question before reading.
//!
//! ## Q1: Ownership & Borrowing
//!
//! ```compile_fail
//! fn main() {
//!     let s = String::from("hello");
//!     let r1 = &s;
//!     let r2 = &s;
//!     let r3 = &mut s; // ERROR: cannot borrow as mutable
//!     println!("{r1} {r2} {r3}");
//! }
//! ```
//!
//! **Answer:** Rust enforces that you cannot have a mutable reference while
//! immutable references exist. Fix: ensure `r1` and `r2` are no longer used
//! before creating `r3`, or restructure to avoid needing both.
//!
//! ## Q2: Lifetimes
//!
//! ```rust
//! fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//!     if x.len() > y.len() { x } else { y }
//! }
//! ```
//!
//! **Answer:** Both parameters and the return type share lifetime `'a`. The
//! compiler infers the returned reference lives at least as long as the shorter
//! of the two input lifetimes.
//!
//! ## Q3: Send & Sync
//!
//! - `Send`: A type can be transferred across thread boundaries.
//! - `Sync`: A type can be shared (via `&T`) across thread boundaries.
//! - Example of Send but not Sync: `Cell<T>`.
//!
//! ## Q4: Blocking in Async
//!
//! Calling a blocking function inside `tokio::spawn` blocks the executor thread.
//! Fix: use `tokio::task::spawn_blocking` or `tokio::time::sleep`.
//!
//! ## Q5: Error Handling
//!
//! - `unwrap()`: Panics on Err. Only use in tests.
//! - `expect("msg")`: Like unwrap but with a message.
//! - `?`: Propagates errors to the caller.
//! - `match`: Full control over both Ok and Err variants.
//!
//! ## Q6: Smart Pointers
//!
//! - `Box<T>`: Heap allocation, single owner.
//! - `Rc<T>`: Reference-counted, single-threaded shared ownership.
//! - `Arc<T>`: Atomic reference-counted, thread-safe shared ownership.
//!
//! ## Q7: Trait Objects vs Generics
//!
//! - `dyn Trait`: Dynamic dispatch, smaller binary, heterogeneous collections.
//! - `impl Trait`: Static dispatch in return position.
//! - `<T: Trait>`: Static dispatch, monomorphized, faster.
//!
//! ## Q8: Interior Mutability
//!
//! - `Cell<T>`: Copy-based get/set, single-threaded.
//! - `RefCell<T>`: Runtime borrow checking, single-threaded.
//! - `Mutex<T>`: Thread-safe mutual exclusion.
//!
//! ## Q9: Pin & Unpin
//!
//! Self-referential types must not be moved after creation. `Pin<P>` guarantees
//! the pointee won't move. Async futures are `!Unpin` because they may contain
//! self-references.
//!
//! ## Q10: Unsafe
//!
//! Three things requiring `unsafe`:
//! 1. Dereferencing a raw pointer
//! 2. Calling an `unsafe fn`
//! 3. Implementing an `unsafe trait`
//!
//! Minimize blast radius: write safe wrappers, document invariants, keep unsafe
//! blocks small.

pub mod api_gateway;
pub mod url_shortener;
