use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::NodeId;

/// Internal representation of a thread-safe graph node.
struct Node<T> {
    value: T,
    edges: Vec<NodeId>,
}

/// A thread-safe directed graph using `Arc<RwLock<>>`.
///
/// `ConcurrentGraph` mirrors the API of [`crate::Graph`] but is safe to
/// share across threads (`Send + Sync`).  Interior state is protected by
/// a `RwLock` so that multiple readers can inspect the graph concurrently
/// while writers get exclusive access.
///
/// Individual node values are further wrapped in `Arc<RwLock<Node<T>>>` so
/// that a caller on one thread can read a node while another thread adds
/// edges elsewhere.
///
/// # Design notes for students
///
/// * Why `RwLock` instead of `Mutex`?  Graphs are read-heavy (traversal),
///   so a reader-writer lock avoids unnecessary contention.
/// * Why `Arc` instead of `Rc`?  `Rc` is *not* `Send`; only `Arc` can
///   cross thread boundaries.
/// * The `T: Send + Sync` bound is required because values of type `T` are
///   shared across threads behind the `Arc<RwLock<...>>`.
pub struct ConcurrentGraph<T: Send + Sync> {
    // TODO: add fields
    //   - thread-safe storage of nodes (e.g. Arc<RwLock<HashMap<...>>>)
    //   - an atomic or mutex-protected counter for NodeId generation
    _marker: std::marker::PhantomData<T>,
}

impl<T: Send + Sync> ConcurrentGraph<T> {
    /// Creates a new, empty concurrent graph.
    pub fn new() -> Self {
        todo!()
    }

    /// Adds a node with the given value and returns its unique `NodeId`.
    ///
    /// This method takes `&self` (not `&mut self`) because interior
    /// mutability makes it safe to call from multiple threads.
    pub fn add_node(&self, value: T) -> NodeId {
        todo!()
    }

    /// Adds a directed edge from `from` to `to`.
    ///
    /// Returns `true` if both nodes exist and the edge was added, or
    /// `false` if either node does not exist.
    pub fn add_edge(&self, from: NodeId, to: NodeId) -> bool {
        todo!()
    }

    /// Returns the list of neighbor `NodeId`s for the given node,
    /// or an empty `Vec` if the node does not exist.
    pub fn neighbors(&self, id: NodeId) -> Vec<NodeId> {
        todo!()
    }

    /// Provides access to a node's value through a callback, similar to
    /// [`crate::Graph::with_node`].
    ///
    /// Returns `None` if the node does not exist.
    pub fn with_node<F, R>(&self, id: NodeId, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        todo!()
    }

    /// Returns the total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        todo!()
    }
}

impl<T: Send + Sync> Default for ConcurrentGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}
