# Building Block 0a: Ownership, Borrowing, and Lifetimes

**Prerequisites**: Basic programming experience in any language. Familiarity with the
command line and a text editor.

Before starting [Project: Ownership Arena](../projects/foundations-1/README.md), complete
the readings and exercises below. These concepts are the foundation of everything in Rust.

## What to read

- [The Rust Book, Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).
  Read all three sections carefully: ownership rules, references/borrowing, and slices.
  This is non-negotiable — you must understand these before writing any Rust.

- [The Rust Book, Chapter 10.3: Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).
  Lifetimes are how Rust tracks how long references are valid. Read this after Chapter 4.

- [Rust By Example: Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html).
  Shorter examples that reinforce the same concepts.

- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
  by pretzelhammer. Excellent blog post that clears up the most frequent confusions.

## Key concepts to internalize

### The Three Rules of Ownership
1. Each value in Rust has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. You can have either one mutable reference OR any number of immutable references
   (but not both at the same time).

### `String` vs `&str`
- `String` is an owned, heap-allocated, growable string. It owns its data.
- `&str` is a borrowed reference to string data. It does not own the data.
- When you need to store a string: use `String`.
- When you need to read a string: accept `&str`.

### Lifetime Elision Rules
The compiler infers lifetimes in most cases. The three elision rules are:
1. Each reference parameter gets its own lifetime.
2. If there's exactly one input lifetime, it's assigned to all output lifetimes.
3. If one of the parameters is `&self` or `&mut self`, its lifetime is assigned to outputs.

When these rules aren't sufficient, you must annotate explicitly.

## Exercises

Before starting the project, try these small exercises in a scratch file:

### Exercise 1: Fix the borrow checker errors

For each snippet, identify why it fails to compile and fix it:

```rust
// Snippet A
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}

// Snippet B
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// Snippet C
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first word is: {}", word);
}
```

### Exercise 2: Annotate lifetimes

Add explicit lifetime annotations to these function signatures (even where the compiler
could infer them). This builds your intuition for how lifetimes flow:

```rust
fn first(s: &str) -> &str { &s[..1] }

fn either(a: &str, b: &str, pick_first: bool) -> &str {
    if pick_first { a } else { b }
}

struct Wrapper { data: &str }

impl Wrapper {
    fn get(&self) -> &str { self.data }
}
```

### Exercise 3: Interior references

Write a struct `Config` that holds a `HashMap<String, String>` and a method
`get(&self, key: &str) -> Option<&str>` that returns a reference to a stored value.
Verify that the returned reference borrows from `self`.

## You're ready when...

- [ ] You can explain the difference between `String`, `&str`, and `&'a str`
- [ ] You can predict when the borrow checker will reject code
- [ ] You understand why functions that return references need lifetime annotations
- [ ] You can fix borrow checker errors without guessing

Next: [Project: Ownership Arena](../projects/foundations-1/README.md)
