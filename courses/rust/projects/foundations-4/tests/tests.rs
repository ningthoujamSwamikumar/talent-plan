use smart_pointer_workshop::{ConcurrentGraph, Counted, Describe, Graph, NodeId};
use std::sync::Arc;
use std::thread;

// ---------------------------------------------------------------------------
// Part 1: Counted<T>
// ---------------------------------------------------------------------------

#[test]
fn counted_initial_count_is_zero() {
    let c = Counted::new(42);
    assert_eq!(c.access_count(), 0);
}

#[test]
fn counted_deref_increments_count() {
    let c = Counted::new(String::from("hello"));
    // Trigger Deref by calling a method on the inner String.
    let _ = c.len();
    assert_eq!(c.access_count(), 1);
    let _ = c.is_empty();
    assert_eq!(c.access_count(), 2);
}

#[test]
fn counted_deref_mut_increments_count() {
    let mut c = Counted::new(vec![1, 2, 3]);
    c.push(4); // triggers DerefMut
    assert_eq!(c.access_count(), 1);
    assert_eq!(c.len(), 4); // triggers Deref
    assert_eq!(c.access_count(), 2);
}

#[test]
fn counted_reset_count() {
    let c = Counted::new(10_u32);
    let _ = *c + 1; // Deref
    let _ = *c + 2; // Deref
    assert_eq!(c.access_count(), 2);
    c.reset_count();
    assert_eq!(c.access_count(), 0);
}

// ---------------------------------------------------------------------------
// Part 2: Graph (Rc<RefCell<>>)
// ---------------------------------------------------------------------------

#[test]
fn graph_add_nodes() {
    let mut g: Graph<&str> = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    assert_ne!(a, b);
    assert_eq!(g.node_count(), 2);
}

#[test]
fn graph_add_edges() {
    let mut g: Graph<&str> = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    assert!(g.add_edge(a, b));
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn graph_neighbors() {
    let mut g: Graph<&str> = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b);
    g.add_edge(a, c);
    let mut nbrs = g.neighbors(a);
    nbrs.sort_by_key(|n| n.0);
    assert_eq!(nbrs, vec![b, c]);
}

#[test]
fn graph_with_node_callback() {
    let mut g: Graph<String> = Graph::new();
    let a = g.add_node(String::from("hello"));
    let len = g.with_node(a, |v| v.len());
    assert_eq!(len, Some(5));
}

#[test]
fn graph_with_node_missing() {
    let g: Graph<i32> = Graph::new();
    let result = g.with_node(NodeId(999), |v| *v);
    assert_eq!(result, None);
}

#[test]
fn graph_cycle_detection_no_cycle() {
    let mut g: Graph<&str> = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b);
    g.add_edge(b, c);
    assert!(!g.has_cycle());
}

#[test]
fn graph_cycle_detection_with_cycle() {
    let mut g: Graph<&str> = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b);
    g.add_edge(b, c);
    g.add_edge(c, a);
    assert!(g.has_cycle());
}

#[test]
fn graph_self_loop() {
    let mut g: Graph<&str> = Graph::new();
    let a = g.add_node("a");
    assert!(g.add_edge(a, a));
    assert!(g.has_cycle());
}

#[test]
fn graph_empty() {
    let g: Graph<i32> = Graph::new();
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(!g.has_cycle());
}

#[test]
fn graph_edge_to_nonexistent_node() {
    let mut g: Graph<&str> = Graph::new();
    let a = g.add_node("a");
    assert!(!g.add_edge(a, NodeId(999)));
}

#[test]
fn graph_default() {
    let g: Graph<i32> = Graph::default();
    assert_eq!(g.node_count(), 0);
}

// ---------------------------------------------------------------------------
// Part 3: ConcurrentGraph (Arc<RwLock<>>)
// ---------------------------------------------------------------------------

#[test]
fn concurrent_graph_basic_operations() {
    let g: ConcurrentGraph<&str> = ConcurrentGraph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    assert!(g.add_edge(a, b));
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.neighbors(a), vec![b]);
}

#[test]
fn concurrent_graph_with_node() {
    let g: ConcurrentGraph<String> = ConcurrentGraph::new();
    let a = g.add_node(String::from("world"));
    let len = g.with_node(a, |v| v.len());
    assert_eq!(len, Some(5));
}

#[test]
fn concurrent_graph_multithread_add_nodes() {
    let g = Arc::new(ConcurrentGraph::<i32>::new());
    let mut handles = vec![];

    for i in 0..10 {
        let g = Arc::clone(&g);
        handles.push(thread::spawn(move || {
            g.add_node(i);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(g.node_count(), 10);
}

#[test]
fn concurrent_graph_multithread_add_edges() {
    let g = Arc::new(ConcurrentGraph::<i32>::new());

    // Pre-populate nodes on the main thread.
    let ids: Vec<NodeId> = (0..5).map(|i| g.add_node(i)).collect();

    let mut handles = vec![];
    for i in 0..4 {
        let g = Arc::clone(&g);
        let from = ids[i];
        let to = ids[i + 1];
        handles.push(thread::spawn(move || {
            g.add_edge(from, to);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Each thread added one edge: 0->1, 1->2, 2->3, 3->4
    for i in 0..4 {
        let nbrs = g.neighbors(ids[i]);
        assert!(nbrs.contains(&ids[i + 1]));
    }
}

#[test]
fn concurrent_graph_default() {
    let g: ConcurrentGraph<i32> = ConcurrentGraph::default();
    assert_eq!(g.node_count(), 0);
}

// ---------------------------------------------------------------------------
// Part 4: graph! macro
// ---------------------------------------------------------------------------

#[test]
fn graph_macro_creates_graph() {
    let g = graph! {
        "a" => "b",
        "b" => "c",
        "c" => "a",
    };
    // The macro should produce a Graph with 3 nodes and 3 edges.
    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 3);
    assert!(g.has_cycle());
}

#[test]
fn graph_macro_empty() {
    let g = graph! {};
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
}

// ---------------------------------------------------------------------------
// Part 5: Derive macro (Describe)
// ---------------------------------------------------------------------------

#[derive(Describe)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Describe)]
struct Config {
    name: String,
    retries: u32,
    verbose: bool,
}

#[test]
fn describe_simple_struct() {
    let p = Point { x: 1.0, y: 2.0 };
    assert_eq!(p.describe(), "Point { x: f64, y: f64 }");
}

#[test]
fn describe_multi_field_struct() {
    let c = Config {
        name: String::from("app"),
        retries: 3,
        verbose: true,
    };
    assert_eq!(
        c.describe(),
        "Config { name: String, retries: u32, verbose: bool }"
    );
}
