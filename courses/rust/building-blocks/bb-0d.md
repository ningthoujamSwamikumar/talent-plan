# Building Block 0d: Smart Pointers, Interior Mutability, and Macros

**Prerequisites**: [Building Block 0c](bb-0c.md) and [Project: Iterator Forge](../projects/foundations-3/README.md).

Before starting [Project: Smart Pointer Workshop](../projects/foundations-4/README.md),
complete the readings and exercises below. These are the tools Rust provides for shared
state, runtime flexibility, and metaprogramming.

## What to read

- [The Rust Book, Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html).
  Read all sections: `Box<T>`, `Deref`, `Drop`, `Rc<T>`, `RefCell<T>`, and reference
  cycles. This is a long chapter — take your time.

- [The Rust Book, Chapter 16.3: Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html).
  `Mutex<T>` and `Arc<T>` for thread-safe shared state.

- [The Rust Book, Chapter 19.6: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html).
  Overview of declarative macros and procedural macros.

- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html).
  The definitive guide to `macro_rules!`. Read at least the first three chapters.

- [Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop).
  Skim the README and the `derive(Builder)` project description. You'll implement
  something similar.

## Key concepts

### Smart Pointer Hierarchy

```
Box<T>         — heap allocation, single owner, compile-time checked
Rc<T>          — reference counted, shared ownership, single-threaded
Arc<T>         — atomic reference counted, shared ownership, thread-safe
Cell<T>        — interior mutability via copy, no runtime cost, no borrowing
RefCell<T>     — interior mutability via runtime borrow checking, single-threaded
Mutex<T>       — interior mutability with locking, thread-safe
RwLock<T>      — read-write lock, multiple readers OR one writer, thread-safe
```

### When to use what

| Need | Use |
|------|-----|
| Heap-allocate a value | `Box<T>` |
| Share ownership (single thread) | `Rc<T>` |
| Share ownership (multi thread) | `Arc<T>` |
| Mutate through shared reference (Copy types) | `Cell<T>` |
| Mutate through shared reference (any type, single thread) | `RefCell<T>` |
| Mutate through shared reference (any type, multi thread) | `Mutex<T>` or `RwLock<T>` |
| Shared + mutable (single thread) | `Rc<RefCell<T>>` |
| Shared + mutable (multi thread) | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |

### The `Deref` Trait

```rust
trait Deref {
    type Target;
    fn deref(&self) -> &Self::Target;
}
```

Implementing `Deref` lets your type be used anywhere a `&Target` is expected (deref
coercion). This is how `String` auto-coerces to `&str` and `Box<T>` auto-coerces to `&T`.

### The `Drop` Trait

```rust
trait Drop {
    fn drop(&mut self);
}
```

Called automatically when a value goes out of scope. Use for cleanup: closing files,
releasing locks, freeing resources. Cannot call `drop()` explicitly on a value — use
`std::mem::drop(value)` instead.

### Declarative Macros (`macro_rules!`)

```rust
macro_rules! vec_of_strings {
    // Match a comma-separated list of expressions
    ($($x:expr),* $(,)?) => {
        vec![$($x.to_string()),*]
    };
}

let v = vec_of_strings!["hello", "world", 42];
// Expands to: vec!["hello".to_string(), "world".to_string(), 42.to_string()]
```

Key fragment types: `$x:expr` (expression), `$x:ident` (identifier), `$x:ty` (type),
`$x:tt` (token tree), `$x:pat` (pattern).

### Procedural Macros

Three kinds:
1. **Derive macros** — `#[derive(MyTrait)]` — generate trait implementations
2. **Attribute macros** — `#[my_attr]` — transform annotated items
3. **Function-like macros** — `my_macro!(...)` — like declarative but more powerful

Derive macros use three crates:
- `syn` — parse Rust code into a syntax tree
- `quote` — generate Rust code from a template
- `proc-macro2` — bridge between the two

## Exercises

### Exercise 1: Rc<RefCell<>> pattern

```rust
use std::rc::Rc;
use std::cell::RefCell;

// Create a shared, mutable counter
let counter = Rc::new(RefCell::new(0));

let c1 = Rc::clone(&counter);
let c2 = Rc::clone(&counter);

*c1.borrow_mut() += 1;
*c2.borrow_mut() += 1;

assert_eq!(*counter.borrow(), 2);
```

Extend this: create a `SharedList<T>` type that wraps `Rc<RefCell<Vec<T>>>` and provides
`push`, `len`, and `get` methods. Create two clones and verify mutations are visible
through both handles.

### Exercise 2: Custom Deref

Create a `CaseInsensitive(String)` wrapper that implements `Deref<Target = str>`.
Then implement `PartialEq` so that `CaseInsensitive("Hello") == CaseInsensitive("HELLO")`.

### Exercise 3: Arc<Mutex<>> with threads

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(vec![]));
let mut handles = vec![];

for i in 0..10 {
    let data = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        data.lock().unwrap().push(i);
    }));
}

for h in handles { h.join().unwrap(); }
let result = data.lock().unwrap();
assert_eq!(result.len(), 10);
```

Verify this works. Then try replacing `Mutex` with `RwLock` and having some threads
only read while others write.

### Exercise 4: Write a declarative macro

Write a `hash_map!` macro that works like:
```rust
let m = hash_map! {
    "key1" => "value1",
    "key2" => "value2",
};
```

### Exercise 5: Explore proc macros

Read through the `syn` crate's [DeriveInput](https://docs.rs/syn/latest/syn/struct.DeriveInput.html)
documentation. Write pseudocode for a derive macro that would generate a `new()` constructor
for any struct, taking each field as a parameter.

## You're ready when...

- [ ] You can choose the right smart pointer for a given scenario
- [ ] You understand `Rc<RefCell<T>>` and `Arc<Mutex<T>>` patterns
- [ ] You can implement `Deref` and `Drop` for custom types
- [ ] You can write declarative macros with repetition patterns
- [ ] You understand the role of `syn`, `quote`, and `proc-macro2`

Next: [Project: Smart Pointer Workshop](../projects/foundations-4/README.md)
