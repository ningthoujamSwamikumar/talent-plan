# Building Block 0c: Iterators, Closures, and Functional Patterns

**Prerequisites**: [Building Block 0b](bb-0b.md) and [Project: Type Machinist](../projects/foundations-2/README.md).

Before starting [Project: Iterator Forge](../projects/foundations-3/README.md), complete
the readings and exercises below. Rust's iterator system is one of its most powerful
features — zero-cost abstractions that compile down to the same code as hand-written loops.

## What to read

- [The Rust Book, Chapter 13: Functional Language Features](https://doc.rust-lang.org/book/ch13-00-functional-features.html).
  Read both sections: closures and iterators. Pay attention to the performance comparison
  at the end.

- [The Rust Book, Chapter 13.1: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html).
  Focus on the three `Fn` traits and how closures capture their environment.

- [Rust By Example: Iterator](https://doc.rust-lang.org/rust-by-example/trait/iter.html).
  Practical examples of implementing the `Iterator` trait.

- [The `Iterator` trait documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
  Skim the provided methods list. You don't need to memorize them all, but know
  what's available: `map`, `filter`, `fold`, `collect`, `enumerate`, `zip`, `chain`,
  `take`, `skip`, `flat_map`, `any`, `all`, `find`, `position`.

## Key concepts

### The Three Closure Traits

```rust
FnOnce  — can be called once. Consumes captured variables by value.
FnMut   — can be called multiple times. Borrows captured variables mutably.
Fn      — can be called multiple times. Borrows captured variables immutably.
```

Every closure implements `FnOnce`. Those that don't move out of captures also
implement `FnMut`. Those that don't mutate captures also implement `Fn`.

When accepting closures as parameters:
- Use `Fn` when you'll call it multiple times and don't need mutation
- Use `FnMut` when you'll call it multiple times and it needs to mutate state
- Use `FnOnce` when you'll call it exactly once

### Lazy Evaluation

Iterators in Rust are lazy — calling `.map()` or `.filter()` creates a new iterator
but does NO work. Work only happens when you consume the iterator with:
- `collect()` — gather results into a collection
- `for_each()` — call a function on each item
- `count()`, `sum()`, `any()`, `all()` — compute a result
- `for` loop — the most common consumer

This means you can chain many operations without creating intermediate collections.

### The `FromIterator` Trait

```rust
trait FromIterator<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
```

This is what `collect()` calls. When you write `iter.collect::<Vec<_>>()`,
Rust calls `Vec::from_iter(iter)`. You can implement it for your own types.

### `IntoIterator`

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

This is what `for` loops call. `for x in collection` desugars to
`for x in collection.into_iter()`. Implementing this makes your type work with `for`.

## Exercises

### Exercise 1: Implement Iterator for a range type

```rust
struct CountDown(u32);

impl Iterator for CountDown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        // Return current value, decrement, stop at 0
        todo!()
    }
}

// Should work:
let v: Vec<u32> = CountDown(5).collect();
assert_eq!(v, vec![5, 4, 3, 2, 1]);
```

### Exercise 2: Closure capture modes

Predict the output or compiler error for each:

```rust
// A
let name = String::from("Alice");
let greet = || println!("Hello, {}!", name);
greet();
greet();
println!("{}", name);  // Does this work?

// B
let mut count = 0;
let mut increment = || { count += 1; count };
println!("{}", increment());
println!("{}", increment());

// C
let name = String::from("Alice");
let consume = || { drop(name); };
consume();
consume();  // Does this work?
```

### Exercise 3: Iterator chains

Rewrite this imperative code as an iterator chain:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let mut result = Vec::new();
for n in &numbers {
    if n % 2 == 0 {
        let doubled = n * 2;
        if doubled > 10 {
            result.push(doubled);
        }
    }
}
// result should be [12, 16, 20]
```

### Exercise 4: Implement FromIterator

Write a `Histogram` type that can be collected from an iterator of strings,
counting the occurrences of each:

```rust
let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
let hist: Histogram = words.into_iter().collect();
assert_eq!(hist.count("apple"), 3);
```

### Exercise 5: Prove laziness

Write an iterator adaptor that logs when each element is produced. Use it to
verify that `.map().filter().take(2)` only processes elements until 2 pass the filter,
not all elements.

## You're ready when...

- [ ] You can implement the `Iterator` trait for custom types
- [ ] You understand `Fn` vs `FnMut` vs `FnOnce` and when each is needed
- [ ] You can chain iterator operations fluently
- [ ] You understand that iterators are lazy and can prove it
- [ ] You can implement `FromIterator` for a custom collection type

Next: [Project: Iterator Forge](../projects/foundations-3/README.md)
