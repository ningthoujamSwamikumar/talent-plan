use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::NodeId;
use std::marker::PhantomData;

/// Internal representation of a graph node.
///
/// Each node holds a value and a list of edges (as `NodeId`s) to its
/// neighbors.  Nodes are wrapped in `Rc<RefCell<...>>` so that multiple
/// parts of the graph can share ownership and mutate the node's edge list.
struct Node<T> {
    value: T,
    edges: Vec<NodeId>,
}

/// A directed graph using `Rc<RefCell<Node>>` for shared, mutable nodes.
///
/// This graph demonstrates *interior mutability*: even though many parts of
/// the graph share ownership of the same node (via `Rc`), any one of them
/// can add edges through `RefCell`.
///
/// **Important caveat:** Because `Rc` uses reference counting, cycles in
/// the graph will cause memory leaks.  Students should verify this by
/// checking that `Rc::strong_count` never drops to zero for nodes in a
/// cycle.
pub struct Graph<T> {
    // TODO: add fields
    //   - a map from NodeId to Rc<RefCell<Node<T>>>
    //   - a counter for generating the next NodeId
    _marker: PhantomData<T>,
}

impl<T> Graph<T> {
    /// Creates a new, empty graph.
    pub fn new() -> Self {
        todo!()
    }

    /// Adds a node with the given value and returns its unique `NodeId`.
    pub fn add_node(&mut self, value: T) -> NodeId {
        todo!()
    }

    /// Adds a directed edge from `from` to `to`.
    ///
    /// Returns `true` if both nodes exist and the edge was added, or
    /// `false` if either node does not exist.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) -> bool {
        todo!()
    }

    /// Returns the list of neighbor `NodeId`s for the given node,
    /// or an empty `Vec` if the node does not exist.
    pub fn neighbors(&self, id: NodeId) -> Vec<NodeId> {
        todo!()
    }

    /// Provides access to a node's value through a callback.
    ///
    /// Because nodes are behind `Rc<RefCell<...>>`, we cannot hand out a
    /// plain `&T` reference (the borrow would not live long enough).
    /// Instead we accept a closure that receives `&T` and returns a value
    /// of any type `R`.
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

    /// Returns the total number of directed edges in the graph.
    pub fn edge_count(&self) -> usize {
        todo!()
    }

    /// Detects whether the graph contains at least one cycle.
    ///
    /// Uses a depth-first search with three-color marking (white / gray /
    /// black).  A back-edge to a gray node means a cycle exists.
    pub fn has_cycle(&self) -> bool {
        todo!()
    }
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}
