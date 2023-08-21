//! Tree data structure with multiple roots.

use crate::{
    arena::Store,
    error::Error,
    index::NodeIdx,
    node::Node,
    relation::{AppendChild, Detach, NexSibling, Relate},
};

/// The Tree arena allocation data structure.
///
/// Its backed by an [`Vec`] of allocated or free entries.
#[derive(Debug, Clone)]
pub struct Tree<T> {
    pub(crate) nodes: Store<Node<T>>,
}

impl<T> Tree<T> {
    /// Create a new tree.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Store::new(),
        }
    }

    /// Create a new tree with a specific parameter.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Store::with_capacity(capacity),
        }
    }

    /// Add a node to the tree, without relations with the other nodes.
    pub fn create_node(&mut self, value: T) -> NodeIdx {
        let index = self.nodes.allocate(Node::new(value));

        NodeIdx::new(index)
    }

    /// Access the node given the index.
    #[must_use]
    pub fn node(&self, idx: NodeIdx) -> Option<&Node<T>> {
        self.nodes.get(idx.index())
    }

    /// Access the node given the index.
    #[must_use]
    pub fn node_mut(&mut self, idx: NodeIdx) -> Option<&mut Node<T>> {
        self.nodes.get_mut(idx.index())
    }

    /// Access the node given the index.
    #[must_use]
    pub fn get(&self, idx: NodeIdx) -> Option<&T> {
        self.node(idx).map(AsRef::as_ref)
    }

    /// Access the node given the index.
    #[must_use]
    pub fn get_mut(&mut self, idx: NodeIdx) -> Option<&mut T> {
        self.node_mut(idx).map(AsMut::as_mut)
    }

    /// Make a node child to the passed parent.
    ///
    /// It will detach the child node and append it as last child of the parent.
    ///
    /// # Example
    ///
    /// ```
    /// let mut tree = forest_ds::Tree::new();
    ///
    /// let a = tree.create_node("a");
    /// let b = tree.create_node("b");
    ///
    /// tree.make_child(a, b);
    /// ```
    pub fn make_child(&mut self, parent: NodeIdx, child: NodeIdx) -> Result<(), Error> {
        AppendChild::new(child, parent).bind(self)?;

        Ok(())
    }

    /// Make two node sibling to each other.
    ///
    /// It will detach the next_sibling node and insert it as the next sibling to the previous one.
    ///
    /// # Example
    ///
    /// ```
    /// let mut tree = forest_ds::Tree::new();
    ///
    /// let a = tree.create_node("a");
    /// let b = tree.create_node("b");
    ///
    /// tree.make_sibling(a, b);
    /// ```
    pub fn make_sibling(
        &mut self,
        prev_sibling: NodeIdx,
        next_sibling: NodeIdx,
    ) -> Result<(), Error> {
        NexSibling::new(next_sibling, prev_sibling).bind(self)?;

        Ok(())
    }

    /// Make a node an orphan.
    ///
    /// It will detach the node from the parent and siblings.
    ///
    /// # Example
    ///
    /// ```
    /// let mut tree = forest_ds::Tree::new();
    ///
    /// let a = tree.create_node("a");
    /// let b = tree.create_node("b");
    ///
    /// tree.detach_node(a, b);
    /// ```
    pub fn detach(&mut self, node: NodeIdx) -> Result<(), Error> {
        Detach::new(node).bind(self)?;

        Ok(())
    }

    /// Removes a node from the three.
    ///
    /// It will detach the node from the parent and siblings, removing it and its children.
    ///
    /// # Example
    ///
    /// ```
    /// let mut tree = forest_ds::Tree::new();
    ///
    /// let a = tree.create_node("a");
    /// let b = tree.create_node("b");
    /// tree.make_child(a, b);
    ///
    /// tree.remove(a);
    /// ```
    pub fn remove(&mut self, node: NodeIdx) -> Result<(), Error> {
        self.detach(node)?;

        todo!("iter the node indexes to remove them")
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}
