# Building Block 0b: Traits, Generics, and Error Handling

**Prerequisites**: [Building Block 0a](bb-0a.md) and [Project: Ownership Arena](../projects/foundations-1/README.md).

Before starting [Project: Type Machinist](../projects/foundations-2/README.md), complete
the readings and exercises below. Rust's trait system is how you write reusable,
composable code — the backbone of every library you'll use.

## What to read

- [The Rust Book, Chapter 10.1-10.2: Generics and Traits](https://doc.rust-lang.org/book/ch10-00-generics.html).
  Read the sections on generic types, trait definitions, trait implementations, and
  trait bounds.

- [The Rust Book, Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html).
  Read all sections. Focus on `Result<T, E>`, the `?` operator, and when to use
  `panic!` vs `Result`.

- [Rust API Guidelines: Type Safety](https://rust-lang.github.io/api-guidelines/type-safety.html).
  How professional Rust code uses the type system for correctness.

- [thiserror documentation](https://docs.rs/thiserror/latest/thiserror/).
  The standard way to define error types in libraries.

- [Rust By Example: Associated Types](https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html).
  Short examples showing when to use associated types vs generic parameters.

## Key concepts

### Traits vs Interfaces

If you come from Java/Go/TypeScript, traits are like interfaces but more powerful:
- They can have default method implementations
- They can have associated types and constants
- They can be implemented for types you don't own (with restrictions)
- They enable operator overloading (`Add`, `Mul`, etc.)
- They can be used as bounds (`fn foo<T: Display>(x: T)`) or as trait objects (`&dyn Display`)

### Associated Types vs Generic Parameters

```rust
// Associated type: one implementation per type
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Generic parameter: multiple implementations per type
trait From<T> {
    fn from(value: T) -> Self;
}
```

Use associated types when there's exactly one natural implementation. Use generic
parameters when a type might implement the trait multiple ways.

### The Error Handling Stack

```
thiserror  — for library error types (derive Error on your enums)
anyhow     — for application error handling (catch-all Result type)
?          — the propagation operator (converts and returns errors)
From<T>    — how error types convert between each other
```

For this project, we use `thiserror` because you're building a library.

### Object Safety

A trait is object-safe (usable as `dyn Trait`) if:
- All methods have `self`, `&self`, or `&mut self` as first parameter
- No methods return `Self`
- No methods have generic type parameters
- No associated functions (methods without `self`)

This matters when you want to store different types that implement the same trait
in a collection.

## Exercises

### Exercise 1: Define and implement a trait

```rust
trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;

    // Default implementation using another method
    fn headline(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

Implement `Summary` for at least two different structs. Verify default methods work.

### Exercise 2: Generic functions with trait bounds

Write a function `print_all<T: Display>(items: &[T])` that prints each item.
Then write `largest<T: PartialOrd>(list: &[T]) -> &T` that returns the largest element.

### Exercise 3: Error type with thiserror

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: expected {expected}, got {got}")]
    Parse { expected: String, got: String },

    #[error("Not found: {0}")]
    NotFound(String),
}
```

Write a function that returns `Result<String, AppError>` and uses the `?` operator
to propagate both `io::Error` and custom errors.

### Exercise 4: From/Into conversions

Implement `From<(f64, f64)>` for a `Point` struct and `From<Point>` for a `(f64, f64)` tuple.
Verify that `Into` works automatically in both directions.

## You're ready when...

- [ ] You can define traits with associated types and default methods
- [ ] You can write generic functions with trait bounds
- [ ] You understand when a trait is object-safe
- [ ] You can define error types with `thiserror` and use `?` for propagation
- [ ] You know when to use `From`/`Into` vs `TryFrom`/`TryInto`

Next: [Project: Type Machinist](../projects/foundations-2/README.md)
