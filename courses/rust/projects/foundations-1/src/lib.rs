#![deny(missing_docs)]
//! A string interner and typed arena demonstrating Rust's ownership model.
//!
//! This crate provides two core data structures:
//!
//! - [`StringInterner`]: deduplicates strings and hands out cheap [`InternId`] handles.
//! - [`Arena`]: bulk-allocates values and returns references tied to the arena's lifetime.
//!
//! Together they illustrate ownership, borrowing, lifetimes, and interior mutability.

use std::{cell::RefCell, collections::HashMap};

// ---------------------------------------------------------------------------
// Part 1 & 2: String Interner
// ---------------------------------------------------------------------------

/// A unique identifier for an interned string.
///
/// `InternId` is a lightweight handle — it is `Copy`, so you can pass it around
/// freely without worrying about ownership.  The actual string data lives inside
/// the [`StringInterner`] that created this ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternId(usize);

impl InternId {
    /// Creates an `InternId` from a raw `usize` index.
    ///
    /// This is mainly useful in tests to construct IDs that may not correspond
    /// to any interned string (e.g., to verify that `get` returns `None`).
    pub fn from_raw(index: usize) -> Self {
        InternId(index)
    }

    /// Returns the underlying `usize` index.
    pub fn as_raw(self) -> usize {
        self.0
    }
}

/// A string interner that deduplicates and stores strings.
///
/// The interner *owns* all string data.  When you call [`intern`](StringInterner::intern),
/// the interner either stores a new `String` or finds the existing copy, and
/// returns a small [`InternId`] handle.
///
/// Looking up a string with [`get`](StringInterner::get) returns a reference
/// (`&str`) that **borrows from the interner**.  This means the reference is
/// only valid as long as the interner is alive — the compiler enforces this
/// for you.
///
/// # Example (after you implement it)
///
/// ```rust,no_run
/// use ownership_arena::{StringInterner, InternId};
///
/// let mut interner = StringInterner::new();
/// let id = interner.intern("hello");
/// assert_eq!(interner.get(id), Some("hello"));
/// ```
pub struct StringInterner {
    // TODO: students add fields
    //
    // Hint: You need two things:
    //   1. A way to store strings and retrieve them by index (`Vec<String>`).
    //   2. A way to check whether a string has already been interned and, if
    //      so, what its index is (`HashMap<String, usize>`).
    //
    // Think about *who owns* each `String`.  The `Vec` owns the canonical
    // copy; the `HashMap` needs to look strings up — consider what key type
    // lets you avoid a second allocation.
    storage: Vec<String>,
    index: HashMap<String, usize>,
}

impl StringInterner {
    /// Creates a new, empty interner.
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Interns a string, returning its unique [`InternId`].
    ///
    /// If the string was already interned, the existing ID is returned and no
    /// new allocation occurs.  If it is new, the interner takes ownership of a
    /// copy and assigns the next sequential ID.
    ///
    /// # Arguments
    ///
    /// * `s` — a string slice.  The interner will allocate its own `String` if
    ///   the value has not been seen before.
    pub fn intern(&mut self, s: &str) -> InternId {
        match self.index.get(s) {
            Some(id) => InternId(*id),
            None => {
                let id = self.storage.len();
                self.storage.push(s.to_string());
                self.index.insert(s.to_string(), id);
                InternId(id)
            }
        }
    }

    /// Looks up a previously interned string by its [`InternId`].
    ///
    /// Returns `None` if the ID does not correspond to any interned string
    /// (e.g., it came from a different interner).
    ///
    /// # Lifetime note
    ///
    /// The returned `&str` borrows from `&self`.  Rust's lifetime-elision
    /// rules automatically give this signature the meaning:
    ///
    /// ```rust,ignore
    /// fn get<'a>(&'a self, id: InternId) -> Option<&'a str>
    /// ```
    ///
    /// So the reference is valid as long as the interner is not dropped or
    /// mutably borrowed.
    pub fn get(&self, id: InternId) -> Option<&str> {
        self.storage.get(id.as_raw()).map(|v| v.as_str())
    }

    /// Returns the number of unique strings currently interned.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if no strings have been interned.
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Part 3: Typed Arena
// ---------------------------------------------------------------------------

/// A typed arena that allocates values and hands out references.
///
/// Unlike a `Vec<T>`, an arena lets you call [`alloc`](Arena::alloc) with a
/// shared `&self` reference (thanks to interior mutability) and returns a
/// reference whose lifetime is tied to the arena.
///
/// # Interior mutability
///
/// The arena stores its values internally using `RefCell<Vec<T>>`.  This lets
/// `alloc` take `&self` instead of `&mut self`, which is important when you
/// want to hand out multiple references that all borrow from the same arena.
///
/// # Safety note
///
/// The references returned by `alloc` remain valid for the lifetime of the
/// arena because `Vec` never moves its elements once allocated (the arena
/// never removes elements).  However, `Vec` *can* reallocate when it grows.
/// To solve this, the arena uses a `Vec<Box<T>>` so that each element is
/// heap-allocated at a stable address.
///
/// # Example (after you implement it)
///
/// ```rust,no_run
/// use ownership_arena::Arena;
///
/// let arena = Arena::new();
/// let x = arena.alloc(42);
/// let y = arena.alloc(99);
/// assert_eq!(*x + *y, 141);
/// ```
pub struct Arena<T> {
    // TODO: students replace this with real fields
    //
    // Hint: You need interior mutability so that `alloc` can take `&self`.
    // Use `RefCell<Vec<Box<T>>>`:
    //   - `RefCell` gives you runtime-checked mutable access from a `&self` method.
    //   - `Box<T>` ensures each value has a stable heap address that won't
    //     move when the Vec grows.
    //
    // After adding the field, remove the PhantomData below.
    storage: RefCell<Vec<Box<T>>>,
}

impl<T> Arena<T> {
    /// Creates a new, empty arena.
    pub fn new() -> Self {
        Self {
            storage: RefCell::new(Vec::new()),
        }
    }

    /// Allocates a value in the arena and returns a shared reference to it.
    ///
    /// The returned reference borrows from `&self`, so it lives as long as
    /// the arena does.
    ///
    /// # Panics
    ///
    /// Panics if the internal `RefCell` is already mutably borrowed (this
    /// should not happen in normal single-threaded use).
    pub fn alloc(&self, value: T) -> &T {
        self.storage.borrow_mut().push(Box::new(value));
        if let Some(last_val) = self.storage.borrow().last() {
            // SAFETY: The Box ensures a stable heap address. The arena never removes
            // or replaces elements, so pointer remains valid for the arena's 
            // lifetime. We tie output lifetime to `&self` (the arena), which is correct.
            unsafe { &*(&**last_val as *const T) }
        }else{
            panic!("Called last on empty vec")
        }
        //
        // Hint (high-level steps):
        //   1. `self.storage.borrow_mut().push(Box::new(value));`
        //   2. Get a shared borrow of the vec.
        //   3. Get a reference to the last element.
        //   4. Convert the `&T` inside the `Ref` guard into a raw pointer,
        //      then back into a reference with the arena's lifetime.
        //      This is safe because the `Box` keeps the value at a stable
        //      address and the arena never drops elements while it's alive.
        //
        // This part requires ONE small `unsafe` block.  The README explains why.
    }

    /// Returns the number of values currently allocated in the arena.
    pub fn len(&self) -> usize {
        self.storage.borrow().len()
    }

    /// Returns `true` if no values have been allocated.
    pub fn is_empty(&self) -> bool {
        self.storage.borrow().is_empty()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Part 4: Cross-References — Document
// ---------------------------------------------------------------------------

/// A document whose fields borrow from a [`StringInterner`].
///
/// The lifetime parameter `'a` tells the compiler: "this `Document` contains
/// references that borrow from something that lives at least as long as `'a`."
/// In practice `'a` will be the lifetime of the `StringInterner` the strings
/// came from.
///
/// # Example (after you implement it)
///
/// ```rust,no_run
/// use ownership_arena::{StringInterner, Document};
///
/// let mut interner = StringInterner::new();
/// let title_id = interner.intern("My Post");
/// let tag_id   = interner.intern("rust");
///
/// let title = interner.get(title_id).unwrap();
/// let tag   = interner.get(tag_id).unwrap();
///
/// let doc = Document::new(title, vec![tag]);
/// assert!(doc.has_tag("rust"));
/// ```
pub struct Document<'a> {
    /// The title, borrowed from a [`StringInterner`].
    pub title: &'a str,
    /// Tags, each borrowed from a [`StringInterner`].
    pub tags: Vec<&'a str>,
}

impl<'a> Document<'a> {
    /// Creates a new document.
    ///
    /// Both `title` and every element of `tags` must borrow from the same
    /// source (or at least live as long as `'a`).
    pub fn new(title: &'a str, tags: Vec<&'a str>) -> Self {
        Self { title, tags }
    }

    /// Returns `true` if the document has been tagged with `tag`.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag)
    }
}
