# Project: Smart Pointer Workshop

**Phase 0, Project 4 (Final Foundations Project)**

Smart pointers, interior mutability, and macros.

## Introduction: Why Smart Pointers Matter

Every backend engineer eventually encounters problems that plain references
cannot solve: shared caches that multiple components read and update,
configuration objects passed across thread boundaries, graphs whose nodes
point to each other in cycles.  Rust's ownership model deliberately makes
these patterns hard with bare references alone -- not because the patterns
are bad, but because they require *explicit* decisions about sharing and
mutation.

Smart pointers are the tools Rust gives you to make those decisions:

| Pointer | When to reach for it |
|---------|---------------------|
| `Box<T>` | Heap allocation, recursive types, trait objects |
| `Rc<T>` | Shared ownership in single-threaded code |
| `Arc<T>` | Shared ownership across threads |
| `RefCell<T>` | Interior mutability (runtime borrow checking) |
| `Mutex<T>` / `RwLock<T>` | Thread-safe interior mutability |

In this project you will use every one of these to build increasingly
powerful graph data structures, and then write macros to reduce boilerplate.

## Project Structure

```
foundations-4/
  Cargo.toml              # main crate
  src/
    lib.rs                # re-exports, Describe trait, graph! macro
    counted.rs            # Part 1
    graph.rs              # Part 2
    concurrent_graph.rs   # Part 3
  derive-describe/        # proc-macro crate (Part 5)
    Cargo.toml
    src/lib.rs
  tests/
    tests.rs              # integration tests
```

## Part 1: Box and Custom Smart Pointers -- `Counted<T>`

**Goal:** Implement a smart pointer that tracks how many times its inner
value is accessed.

### Background

`Box<T>` is the simplest smart pointer: it allocates `T` on the heap and
implements `Deref` so you can use it like a regular reference.  The `Deref`
and `DerefMut` traits are what make the "smart" in smart pointer -- they
let the compiler automatically coerce `&SmartPointer<T>` into `&T`.

`Drop` gives you a destructor: code that runs when the value goes out of
scope.

### Your Task

Fill in the `Counted<T>` struct in `src/counted.rs`:

1. **Fields:** Store the inner `T` and a `Cell<usize>` counter.  `Cell` is
   needed because `Deref::deref(&self)` only gives you `&self`, yet you
   still need to increment the counter.

2. **`Deref` / `DerefMut`:** Each implementation should increment the
   counter and then return a reference to the inner value.

3. **`reset_count`:** Sets the counter back to zero.

4. **`Drop`:** Optionally print or log the final access count.

### Key Concepts

- `Deref` coercion: the compiler calls `deref()` automatically.
- `Cell<T>` provides interior mutability for `Copy` types without runtime
  cost.
- `Drop` ordering: Rust drops fields in declaration order.

## Part 2: Reference-Counted Graph -- `Graph<T>`

**Goal:** Build a directed graph whose nodes can form cycles.

### Background

In many data structures, multiple owners need to point to the same node.
`Rc<T>` (Reference Counted) enables shared ownership in single-threaded
code by keeping a count of how many `Rc` handles exist; the inner value is
dropped only when the last handle is dropped.

But `Rc<T>` only gives you `&T` -- it cannot give you `&mut T` because
other handles might be reading.  `RefCell<T>` solves this by moving the
borrow check to *runtime*: you call `.borrow()` for `&T` and
`.borrow_mut()` for `&mut T`, and it panics if the rules are violated.

The combination `Rc<RefCell<T>>` is the idiomatic way to have
shared + mutable data on a single thread.

### Your Task

Fill in `src/graph.rs`:

1. **Internal `Node<T>`:** Holds a value and a `Vec<NodeId>` of outgoing
   edges.

2. **`Graph<T>` fields:** A `HashMap<NodeId, Rc<RefCell<Node<T>>>>` and a
   counter for generating new `NodeId`s.

3. **`add_node`:** Insert a new node, return its ID.

4. **`add_edge`:** Borrow the `from` node mutably (via `RefCell`) and push
   `to` onto its edge list.  Return `false` if either node is missing.

5. **`neighbors`:** Borrow the node immutably and clone its edge list.

6. **`with_node`:** Because nodes are behind `Rc<RefCell<...>>`, you
   cannot return `&T`.  Instead, accept a closure `F: FnOnce(&T) -> R`
   and call it while holding the borrow.

7. **`has_cycle`:** Implement cycle detection using DFS with three-color
   marking (white = unvisited, gray = in current path, black = finished).
   A back-edge to a gray node means a cycle exists.

### Why Cycles Leak

`Rc` drops the inner value when the strong count reaches zero.  In a
cycle (A -> B -> A), each node keeps the other alive, so the count never
reaches zero.  Rust provides `Weak<T>` to break cycles, but in this
project we intentionally leave the leak to make the point.

## Part 3: Thread-Safe Graph -- `ConcurrentGraph<T>`

**Goal:** Make the graph safe for concurrent access from multiple threads.

### Background

| Single-threaded | Multi-threaded |
|-----------------|---------------|
| `Rc<T>` | `Arc<T>` |
| `RefCell<T>` | `Mutex<T>` or `RwLock<T>` |

`Arc` (Atomically Reference Counted) uses atomic operations instead of
plain integers, making it `Send + Sync`.

`Mutex<T>` provides exclusive access (`lock()` returns a guard that derefs
to `&mut T`).  `RwLock<T>` allows multiple readers or one writer.

The marker traits `Send` and `Sync` are auto-implemented by the compiler:
- `T: Send` means `T` can be moved to another thread.
- `T: Sync` means `&T` can be shared across threads (i.e., `T` can be
  accessed from multiple threads simultaneously).

### Your Task

Fill in `src/concurrent_graph.rs`:

1. Use `Arc<RwLock<HashMap<...>>>` for the node storage and an
   `Arc<Mutex<usize>>` (or `AtomicUsize`) for the ID counter.

2. Implement the same operations as `Graph`, but with `&self` signatures
   (no `&mut self` needed because interior mutability handles mutation).

3. Ensure `ConcurrentGraph<T>` is `Send + Sync` (the compiler will check
   this for you given the right bounds).

### Key Concepts

- Lock granularity: locking the entire map vs. per-node locks.
- Deadlock avoidance: always acquire locks in a consistent order.
- `RwLock` vs. `Mutex`: prefer `RwLock` for read-heavy workloads.

## Part 4: Declarative Macros -- `graph!`

**Goal:** Write a `macro_rules!` macro for convenient graph construction.

### Background

Rust's declarative macros (`macro_rules!`) operate on token trees using
pattern matching.  Key concepts:

- **Fragment specifiers:** `$x:expr`, `$x:ident`, `$x:ty`, etc.
- **Repetitions:** `$($x:expr),*` matches zero or more comma-separated
  expressions.
- **Hygiene:** Macros cannot accidentally capture variables from the
  calling scope (mostly -- Rust's macro hygiene is "partially hygienic").

### Your Task

In `src/lib.rs`, implement the `graph!` macro so that:

```rust
let g = graph! {
    "a" => "b",
    "b" => "c",
    "c" => "a",
};
```

produces a `Graph<&str>` with three nodes and three edges.

Hints:
- You will need to track which values have already been added as nodes.
  Consider using a `HashMap<_, NodeId>` inside the macro expansion.
- The macro body can contain arbitrary Rust code wrapped in `{ ... }`.

## Part 5: Derive Macro -- `#[derive(Describe)]`

**Goal:** Write a procedural macro that auto-generates a `describe()`
method.

### Background

Procedural macros are Rust functions that transform a `TokenStream` into
another `TokenStream`.  They live in a separate crate with
`proc-macro = true`.  The three libraries you will use:

| Crate | Purpose |
|-------|---------|
| `proc_macro` | Compiler-provided `TokenStream` type |
| `syn` | Parses `TokenStream` into a syntax tree (`DeriveInput`) |
| `quote` | Generates `TokenStream` from quasi-quoted Rust code |

### Your Task

In `derive-describe/src/lib.rs`:

1. Parse the input with `syn::parse_macro_input!(input as DeriveInput)`.
2. Extract the struct name and its named fields.
3. Use `quote!` to generate an `impl Describe for #name` block.
4. The `describe()` method should return a string like:
   `"Point { x: f64, y: f64 }"` -- the struct name followed by each
   field's name and type.

Example implementation sketch:

```rust
let fields_desc: Vec<String> = fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    format!("{}: {}", quote!(#name), quote!(#ty))
}).collect();
let body = format!("{} {{ {} }}", struct_name, fields_desc.join(", "));
```

### Important Note

The `Describe` *trait* is defined in the main crate (`src/lib.rs`).  The
derive macro generates code that references `smart_pointer_workshop::Describe`.
Make sure the trait is in scope wherever `#[derive(Describe)]` is used.

## Part 6: Putting It Together

Once all parts compile and tests pass, try this exercise (not graded):

```rust
use std::sync::Arc;
use std::thread;
use smart_pointer_workshop::*;

#[derive(Describe)]
struct City {
    name: String,
    population: u64,
}

fn main() {
    let g = Arc::new(ConcurrentGraph::<City>::new());

    let sf = g.add_node(City { name: "SF".into(), population: 870_000 });
    let la = g.add_node(City { name: "LA".into(), population: 3_900_000 });
    let sea = g.add_node(City { name: "Seattle".into(), population: 750_000 });

    g.add_edge(sf, la);
    g.add_edge(la, sea);
    g.add_edge(sea, sf);

    let handles: Vec<_> = [sf, la, sea].iter().map(|&id| {
        let g = Arc::clone(&g);
        thread::spawn(move || {
            g.with_node(id, |city| {
                println!("{} -- {}", city.describe(), city.population);
            });
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
}
```

## Running the Tests

```bash
cargo test
```

All 22 tests should pass once every `todo!()` is replaced with a working
implementation.

## Checklist

- [ ] `Counted<T>` compiles and tracks accesses
- [ ] `Graph<T>` supports add/query/cycle-detection
- [ ] `ConcurrentGraph<T>` works from multiple threads
- [ ] `graph!` macro builds a graph from DSL syntax
- [ ] `#[derive(Describe)]` generates correct output
- [ ] `cargo test` passes all tests
- [ ] `cargo clippy` is warning-free
