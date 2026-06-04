# Project: Procedural Macro Workshop

**Phase 0 — Rust Foundations**

In this project you will build three procedural macros from scratch: a derive
macro, an attribute macro, and a function-like macro. By the end, you will
understand how Rust's most powerful metaprogramming tool works — the same tool
that powers `serde`, `tokio`, `thiserror`, and most production Rust libraries.

## Prerequisites

- Completion of foundations-1 through foundations-4
- Building block bb-0e (Procedural Macros)

## Deliverables

- [ ] A `Builder` derive macro that generates builder pattern code
- [ ] A `#[sorted]` attribute macro that enforces alphabetical enum variants
- [ ] A `seq!` function-like macro for compile-time loop unrolling
- [ ] Tests passing for all three macros
- [ ] Understanding of `syn`, `quote`, and `proc_macro2`

## Part 1: Set Up the Workspace

Create a Cargo workspace with three crates:

```
my-macros/
├── Cargo.toml (workspace)
├── macros/
│   ├── Cargo.toml    # proc-macro = true
│   └── src/lib.rs
├── usage/
│   ├── Cargo.toml    # depends on macros
│   └── src/main.rs
└── tests/
    └── *.rs
```

The `macros/` crate is the proc macro crate. It must have `proc-macro = true`
in `Cargo.toml` and depend on `syn`, `quote`, and `proc-macro2`. The `usage/`
crate depends on `macros/` and is where you test your macros.

## Part 2: Builder Derive Macro

Implement `#[derive(Builder)]` that generates a builder struct and methods.

**Input:**
```rust
#[derive(Builder)]
struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
```

**Generated code should enable:**
```rust
let cmd = Command::builder()
    .executable("cargo".to_string())
    .args(vec!["test".to_string()])
    .env(vec![])
    .build()
    .unwrap();
```

**Implementation steps:**

1. Parse the input struct using `syn::DeriveInput`.
2. Extract field names and types from `syn::Fields::Named`.
3. Generate a `CommandBuilder` struct with `Option<T>` for each field.
4. Generate setter methods that take `T` and set `Option<T>`.
5. Generate a `build()` method that returns `Result<Command, String>`,
   returning an error if any required field is `None`.
6. Handle `Option<T>` fields specially — they should default to `None`
   without requiring the setter to be called.

**Tests to pass:**
- Builder creates a valid struct when all fields set
- Builder returns error when required field missing
- Optional fields default to `None`
- Setter methods return `&mut Self` for chaining

## Part 3: Sorted Attribute Macro

Implement `#[sorted]` that produces a compile error if enum variants are not
in alphabetical order.

**Input:**
```rust
#[sorted]
enum Error {
    Io(std::io::Error),       // 'I' before 'M' — ok
    Mismatch,                 // 'M' before 'P' — ok
    Parse(String),            // 'P' — ok
}
```

**Error case:**
```rust
#[sorted]
enum Error {
    Parse(String),            // 'P' before 'I' — COMPILE ERROR
    Io(std::io::Error),
    Mismatch,
}
```

**Implementation steps:**

1. Parse the enum with `syn::ItemEnum`.
2. Extract variant names as strings.
3. Check if they're sorted. If not, produce a compile error using
   `syn::Error::new_spanned` pointing at the out-of-order variant.
4. Return the enum unchanged (the attribute only checks, doesn't transform).

**Tests to pass:**
- Sorted enum compiles successfully
- Unsorted enum produces a clear compile error with the offending variant name
- Works with tuple variants, struct variants, and unit variants

## Part 4: Seq Function-Like Macro

Implement `seq!` for compile-time repetition with a counter variable.

**Input:**
```rust
seq!(N in 0..4 {
    fn f~N() -> u32 { N }
});
```

**Generated:**
```rust
fn f0() -> u32 { 0 }
fn f1() -> u32 { 1 }
fn f2() -> u32 { 2 }
fn f3() -> u32 { 3 }
```

**Implementation steps:**

1. Parse the macro input: identifier (`N`), `in`, range (`0..4`), braced body.
2. For each value in the range, substitute:
   - `~N` in identifiers → the concatenated identifier (e.g., `f~N` → `f0`)
   - `N` as an expression → the numeric literal
3. Concatenate all generated code.

This is the most challenging macro. The `~` substitution in identifiers requires
walking the token tree and joining tokens.

**Tests to pass:**
- Generates the correct number of items
- Identifier substitution works (`f~N` → `f0`, `f1`, ...)
- Expression substitution works (`N` → `0`, `1`, ...)
- Works with different ranges

## Part 5: Error Quality

Go back to each macro and improve error messages:

1. **Builder**: If a field type isn't `Clone`, produce a helpful error explaining
   why `Clone` is needed (or remove the `Clone` requirement).
2. **Sorted**: The error should say "Error::Parse should sort before Error::Io"
   not just "variants not sorted."
3. **Seq**: If the range is invalid (e.g., `5..2`), produce a clear error.

Test that your error messages appear at the correct span (highlighting the
right code in the IDE/terminal).

## Part 6: Cargo Expand

Use `cargo expand` to inspect the output of all three macros:

```bash
cargo install cargo-expand
cargo expand --lib  # see all expanded code
```

Verify the generated code is correct, well-formatted, and doesn't produce
clippy warnings.

## Testing

```
cargo test --workspace
```

## What You Will Learn

- How Rust's proc macro system works at the token level
- Parsing Rust syntax with `syn` (structs, enums, token trees)
- Generating Rust code with `quote` (interpolation, repetition)
- Producing quality compile errors from macros with correct spans
- The three types of proc macros and when to use each
- How to debug macros with `cargo expand`
- The architecture behind `serde`, `thiserror`, and other derive macros
