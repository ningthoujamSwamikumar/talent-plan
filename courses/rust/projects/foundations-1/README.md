# Foundations Project 1: Ownership Arena

**Task**: Build a string interner and a typed arena allocator that demonstrate
Rust's ownership model, borrowing rules, and lifetime annotations.

**Goals**:

- Understand the difference between owned data (`String`) and borrowed data (`&str`)
- Learn how Rust's borrow checker prevents use-after-free and dangling references
- Practice writing lifetime annotations on structs and methods
- Use interior mutability (`RefCell`) to build an ergonomic arena API
- Write compile-fail tests with `trybuild` to verify safety guarantees

**Topics**: ownership, borrowing, lifetimes, lifetime elision, interior mutability,
`RefCell`, `HashMap`, `PhantomData`, compile-fail testing.

- [Introduction](#user-content-introduction)
- [Project setup](#user-content-project-setup)
- [Part 1: The String Interner](#user-content-part-1-the-string-interner)
- [Part 2: Lifetime-Annotated Lookups](#user-content-part-2-lifetime-annotated-lookups)
- [Part 3: The Typed Arena](#user-content-part-3-the-typed-arena)
- [Part 4: Cross-References with Documents](#user-content-part-4-cross-references-with-documents)
- [Part 5: Compile-Fail Tests](#user-content-part-5-compile-fail-tests)


## Introduction

Ownership is the heart of Rust. Every value in Rust has exactly one owner, and
when that owner goes out of scope the value is dropped. References let you
*borrow* a value without taking ownership, but the compiler enforces strict
rules: you can have either one mutable reference **or** any number of shared
references, and no reference may outlive the data it points to.

These rules prevent entire classes of bugs — dangling pointers, double frees,
data races — at compile time with zero runtime cost. But they also take
practice to work with fluently.

In this project you will build two data structures that exercise every aspect of
the ownership model:

1. **A string interner** that owns a set of deduplicated strings and hands out
   cheap integer handles. When you look up a string by its handle you get a
   `&str` that *borrows from the interner* — the compiler will not let you use
   that reference after the interner is dropped.

2. **A typed arena** that allocates values of any type and returns references
   with the arena's lifetime. This requires interior mutability so that you can
   keep allocating from a shared `&self` reference.

Along the way you will also build a `Document` struct that holds references into
an interner, requiring you to write explicit lifetime annotations, and you will
write compile-fail tests to prove that the borrow checker rejects unsafe code.


## Project setup

The starter code is in `src/lib.rs`. It contains struct definitions and method
signatures with `todo!()` bodies. Your job is to fill them in so that all tests
pass.

Make sure the project compiles (with expected `todo!` panics) before you start:

```
cargo check
```

Run a specific test with:

```
cargo test <test_name>
```

Run all tests (they will all fail initially):

```
cargo test
```


## Part 1: The String Interner

**Goal**: implement `StringInterner::new`, `intern`, `len`, and `is_empty`.

A string interner stores a set of strings and assigns each unique string a
small integer identifier (`InternId`). If you intern the same string twice, you
get the same ID both times — no duplicate allocation occurs.

### What to do

Open `src/lib.rs` and find the `StringInterner` struct. Replace the `_private`
placeholder field with real storage:

```rust
pub struct StringInterner {
    strings: Vec<String>,
    lookup: HashMap<String, usize>,
}
```

Then implement:

- `new()` — create empty storage.
- `intern(&mut self, s: &str) -> InternId` — if `s` is already in `lookup`,
  return the existing ID. Otherwise, push a new `String` into `strings`, record
  the mapping in `lookup`, and return the new ID.
- `len()` — return `strings.len()`.
- `is_empty()` — return `strings.is_empty()`.

### Ownership insight: `&str` vs `String`

Notice that `intern` takes `&str` (a borrowed string slice) but the interner
stores `String` (an owned, heap-allocated string). The caller does not give up
ownership of anything — the interner creates its own copy via `s.to_owned()` or
`s.to_string()` when it needs to store a new string.

The `HashMap<String, usize>` key is *also* an owned `String`. This means the
interner holds two copies of each string: one in the `Vec` and one as a
`HashMap` key. That is fine for this project. (Advanced: you could avoid the
duplication with unsafe code or by using indices, but that is beyond our scope.)

### Tests to pass

```
cargo test interner_new_is_empty
cargo test interner_intern_single_string
cargo test interner_deduplicates
cargo test interner_multiple_strings
cargo test interner_mixed_dedup_and_new
cargo test interner_empty_string
cargo test interner_default_is_empty
```


## Part 2: Lifetime-Annotated Lookups

**Goal**: implement `StringInterner::get`.

### What to do

Implement `get(&self, id: InternId) -> Option<&str>`. Use the inner `usize` of
the `InternId` (via `id.as_raw()`) to index into the `Vec<String>`, converting
the `&String` to `&str`.

```rust
pub fn get(&self, id: InternId) -> Option<&str> {
    self.strings.get(id.as_raw()).map(|s| s.as_str())
}
```

### Lifetime elision

You did not write any lifetime annotations, yet the compiler knows that the
returned `&str` borrows from `&self`. This is **lifetime elision** — Rust
applies a set of rules to infer lifetimes when the intention is unambiguous:

1. Each input reference gets its own lifetime parameter.
2. If there is exactly one input lifetime, it is assigned to all output
   references.
3. If one of the inputs is `&self` or `&mut self`, the lifetime of `self` is
   assigned to all output references.

Rule 3 applies here: `get` takes `&self`, so the output `&str` gets the
lifetime of `self`. Written explicitly:

```rust
fn get<'a>(&'a self, id: InternId) -> Option<&'a str>
```

The upshot: if the interner is dropped, any `&str` you obtained from `get`
becomes invalid, and the compiler will tell you at compile time.

### Tests to pass

All Part 1 tests still pass, plus:

```
cargo test interner_get_invalid_id_returns_none
```

(The Part 1 tests already call `get`, so they exercise this too.)


## Part 3: The Typed Arena

**Goal**: implement `Arena<T>` — `new`, `alloc`, `len`, and `is_empty`.

An arena allocator owns a collection of values and hands out references to them.
Unlike a `Vec`, the arena guarantees that references remain valid even after more
values are allocated.

### What to do

Replace the `_marker: PhantomData<T>` field with real storage:

```rust
use std::cell::RefCell;

pub struct Arena<T> {
    storage: RefCell<Vec<Box<T>>>,
}
```

Key design decisions:

- **`RefCell`**: We want `alloc` to take `&self` (not `&mut self`) so that
  callers can hold multiple references into the arena at the same time. `RefCell`
  gives us *interior mutability* — runtime-checked mutable access through a
  shared reference.

- **`Box<T>`**: A plain `Vec<T>` reallocates its buffer when it grows, which
  would invalidate all existing references. By storing `Box<T>`, each value
  lives at its own stable heap address. The `Vec` only stores pointers, so
  growing the `Vec` moves pointers around but the values they point to stay put.

### Implementing `alloc`

This method requires a small `unsafe` block. Here is the pattern:

```rust
pub fn alloc(&self, value: T) -> &T {
    let mut storage = self.storage.borrow_mut();
    storage.push(Box::new(value));
    let ptr: *const T = &**storage.last().unwrap();
    // SAFETY: The Box ensures a stable heap address. The arena never removes
    // or replaces elements, so the pointer remains valid for the arena's
    // lifetime. We tie the output lifetime to `&self` (the arena), which
    // is correct.
    unsafe { &*ptr }
}
```

Why is `unsafe` needed? The `borrow_mut()` guard (`RefMut`) is dropped at the
end of `alloc`, so we cannot return a reference derived from it directly. We
convert to a raw pointer and back, asserting that the pointer remains valid.
This is safe because:

1. The `Box` heap allocation is never moved.
2. The arena never removes elements.
3. The returned reference's lifetime is tied to `&self` (the arena), so it
   cannot outlive the data.

### Tests to pass

```
cargo test arena_new_is_empty
cargo test arena_alloc_single
cargo test arena_alloc_multiple
cargo test arena_references_remain_valid_after_growth
cargo test arena_with_different_types
cargo test arena_default_is_empty
```


## Part 4: Cross-References with Documents

**Goal**: implement `Document::new` and `Document::has_tag`.

The `Document` struct holds a title and a list of tags, all borrowed as `&str`
from a `StringInterner`. The struct already has the correct lifetime annotation:

```rust
pub struct Document<'a> {
    pub title: &'a str,
    pub tags: Vec<&'a str>,
}
```

The `'a` says: "this `Document` contains references that must not outlive
whatever they borrow from." When you create a `Document` from interner lookups,
`'a` is bound to the interner's lifetime.

### What to do

Implement:

- `new(title: &'a str, tags: Vec<&'a str>) -> Self` — construct the struct.
- `has_tag(&self, tag: &str) -> bool` — return whether `tag` is in `self.tags`.

Note that `has_tag` takes a second reference (`tag: &str`) with a *different*
lifetime than `'a`. Rust gives it its own anonymous lifetime. The two lifetimes
are independent: `tag` does not need to come from the same interner, and its
lifetime does not affect the `Document`.

### Tests to pass

```
cargo test document_creation
cargo test document_has_tag_returns_false_for_missing
cargo test document_multiple_tags
```


## Part 5: Compile-Fail Tests

**Goal**: verify that the borrow checker rejects code that would be unsafe.

Passing tests prove your code works when used correctly. Compile-fail tests
prove that your code *refuses to compile* when used incorrectly. This is just as
important for a library that relies on the type system for safety.

We use the [`trybuild`](https://docs.rs/trybuild) crate. It compiles small
standalone programs and checks that they produce the expected compiler errors.

### What is already provided

- `tests/compile_fail.rs` — a test harness that runs `trybuild` on every file
  in the `compile-fail/` directory.
- `compile-fail/use_after_drop.rs` — a program that tries to use a `&str`
  reference after the `StringInterner` it came from has been dropped.

### Run it

```
cargo test compile_fail_tests
```

This test passes when the file in `compile-fail/` fails to compile (as
expected). If your interner's lifetime annotations are correct, the borrow
checker will reject the code and `trybuild` will report success.

### Extension: write your own compile-fail test

Create a new file in `compile-fail/` that tests a different invalid pattern.
Ideas:

- Try to return a `&str` from a function where the interner is a local variable.
- Try to mutate a `StringInterner` while holding a reference from `get`.
- Try to use an arena reference after the arena is dropped.

Each file should be a standalone `fn main()` program that uses
`ownership_arena`.

### Tests to pass

```
cargo test compile_fail_tests
```


---

After all five parts are complete, run the full test suite:

```
cargo test
```

All tests should pass. Congratulations — you have built two non-trivial data
structures that leverage Rust's ownership model for memory safety without a
garbage collector!
