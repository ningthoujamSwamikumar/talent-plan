use ownership_arena::{Arena, Document, InternId, StringInterner};

// ---------------------------------------------------------------------------
// StringInterner — basics
// ---------------------------------------------------------------------------

#[test]
fn interner_new_is_empty() {
    let interner = StringInterner::new();
    assert!(interner.is_empty());
    assert_eq!(interner.len(), 0);
}

#[test]
fn interner_intern_single_string() {
    let mut interner = StringInterner::new();
    let id = interner.intern("hello");
    assert_eq!(interner.len(), 1);
    assert!(!interner.is_empty());
    assert_eq!(interner.get(id), Some("hello"));
}

#[test]
fn interner_deduplicates() {
    let mut interner = StringInterner::new();
    let id1 = interner.intern("hello");
    let id2 = interner.intern("hello");
    assert_eq!(id1, id2);
    assert_eq!(interner.len(), 1);
}

#[test]
fn interner_multiple_strings() {
    let mut interner = StringInterner::new();
    let a = interner.intern("alpha");
    let b = interner.intern("beta");
    let c = interner.intern("gamma");
    assert_eq!(interner.len(), 3);
    assert_eq!(interner.get(a), Some("alpha"));
    assert_eq!(interner.get(b), Some("beta"));
    assert_eq!(interner.get(c), Some("gamma"));
}

#[test]
fn interner_get_invalid_id_returns_none() {
    let interner = StringInterner::new();
    assert_eq!(interner.get(InternId::from_raw(999)), None);
}

#[test]
fn interner_mixed_dedup_and_new() {
    let mut interner = StringInterner::new();
    let a = interner.intern("x");
    let b = interner.intern("y");
    let c = interner.intern("x"); // duplicate
    let d = interner.intern("z");
    assert_eq!(a, c);
    assert_ne!(a, b);
    assert_ne!(b, d);
    assert_eq!(interner.len(), 3);
}

#[test]
fn interner_empty_string() {
    let mut interner = StringInterner::new();
    let id = interner.intern("");
    assert_eq!(interner.get(id), Some(""));
    assert_eq!(interner.len(), 1);
}

#[test]
fn interner_default_is_empty() {
    let interner = StringInterner::default();
    assert!(interner.is_empty());
}

// ---------------------------------------------------------------------------
// Arena — basics
// ---------------------------------------------------------------------------

#[test]
fn arena_new_is_empty() {
    let arena: Arena<i32> = Arena::new();
    assert!(arena.is_empty());
    assert_eq!(arena.len(), 0);
}

#[test]
fn arena_alloc_single() {
    let arena = Arena::new();
    let r = arena.alloc(42);
    assert_eq!(*r, 42);
    assert_eq!(arena.len(), 1);
}

#[test]
fn arena_alloc_multiple() {
    let arena = Arena::new();
    let a = arena.alloc(1);
    let b = arena.alloc(2);
    let c = arena.alloc(3);
    assert_eq!(*a, 1);
    assert_eq!(*b, 2);
    assert_eq!(*c, 3);
    assert_eq!(arena.len(), 3);
}

#[test]
fn arena_references_remain_valid_after_growth() {
    let arena = Arena::new();
    let first = arena.alloc(String::from("first"));
    // Allocate enough to likely trigger a Vec reallocation
    for i in 0..100 {
        arena.alloc(format!("item-{}", i));
    }
    // The first reference must still be valid
    assert_eq!(first.as_str(), "first");
    assert_eq!(arena.len(), 101);
}

#[test]
fn arena_with_different_types() {
    let int_arena: Arena<u64> = Arena::new();
    let str_arena: Arena<String> = Arena::new();

    let n = int_arena.alloc(7);
    let s = str_arena.alloc(String::from("hello"));

    assert_eq!(*n, 7);
    assert_eq!(s.as_str(), "hello");
}

#[test]
fn arena_default_is_empty() {
    let arena: Arena<bool> = Arena::default();
    assert!(arena.is_empty());
}

// ---------------------------------------------------------------------------
// Document — cross-references
// ---------------------------------------------------------------------------

#[test]
fn document_creation() {
    let mut interner = StringInterner::new();
    let title_id = interner.intern("My Post");
    let tag_id = interner.intern("rust");

    let title = interner.get(title_id).unwrap();
    let tag = interner.get(tag_id).unwrap();

    let doc = Document::new(title, vec![tag]);
    assert_eq!(doc.title, "My Post");
    assert!(doc.has_tag("rust"));
}

#[test]
fn document_has_tag_returns_false_for_missing() {
    let mut interner = StringInterner::new();
    let t = interner.intern("Title");
    let title = interner.get(t).unwrap();

    let doc = Document::new(title, vec![]);
    assert!(!doc.has_tag("missing"));
}

#[test]
fn document_multiple_tags() {
    let mut interner = StringInterner::new();
    let t = interner.intern("Guide");
    let t1 = interner.intern("rust");
    let t2 = interner.intern("ownership");
    let t3 = interner.intern("beginner");

    let title = interner.get(t).unwrap();
    let tags: Vec<&str> = [t1, t2, t3]
        .iter()
        .map(|id| interner.get(*id).unwrap())
        .collect();

    let doc = Document::new(title, tags);
    assert!(doc.has_tag("rust"));
    assert!(doc.has_tag("ownership"));
    assert!(doc.has_tag("beginner"));
    assert!(!doc.has_tag("advanced"));
}
