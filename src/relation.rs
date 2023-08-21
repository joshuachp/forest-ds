//! Make nodes in related between each others.

use crate::{index::NodeIdx, tree::Tree};

/// Error returned by the [`Relate`] trait.
#[derive(Debug, Clone, Copy, thiserror::Error, displaydoc::Display)]
pub enum RelationError {
    /// couldn't get the node from the index
    Invalid,
}

/// Trait to make a relation
pub(crate) trait Relate<T> {
    fn bind(self, tree: &mut Tree<T>) -> Result<(), RelationError>;

    fn link_siblings(tree: &mut Tree<T>, next_sibling: Option<usize>, prev_sibling: Option<usize>) {
        // Set the next sibling, if prev is none this is the first
        let next = next_sibling.and_then(|next| tree.nodes.get_mut(next));
        if let Some(node) = next {
            node.prev_sibling = prev_sibling;
        }
        // Set the prev sibling, if next is none this is the last
        let prev = prev_sibling.and_then(|prev| tree.nodes.get_mut(prev));
        if let Some(node) = prev {
            node.next_sibling = next_sibling;
        }
    }
}

/// Relate a node to the previous sibling
pub(crate) struct NexSibling {
    node: NodeIdx,
    prev_sibling: NodeIdx,
}

impl NexSibling {
    pub(crate) fn new(node: NodeIdx, prev_sibling: NodeIdx) -> Self {
        Self { node, prev_sibling }
    }
}

impl<T> Relate<T> for NexSibling {
    fn bind(self, tree: &mut Tree<T>) -> Result<(), RelationError> {
        // Detach the node before making the new relation
        // This will also make sure the node exists
        Detach::new(self.node).bind(tree)?;

        // Get the node indexes
        let sibling = tree
            .node_mut(self.prev_sibling)
            .ok_or(RelationError::Invalid)?;
        let parent = sibling.parent;
        let old_next_sibling = sibling.next_sibling;

        let node_idx = Some(self.node.index());
        Self::link_siblings(tree, node_idx, Some(self.prev_sibling.index()));
        Self::link_siblings(tree, old_next_sibling, node_idx);

        // Make the node child of the parent updating first or last child index
        let parent = parent.and_then(|parent_idx| tree.nodes.get_mut(parent_idx));
        if let Some(node) = parent {
            // Replace last child with next sibling
            if node.last_child == Some(self.prev_sibling.index()) {
                node.last_child = Some(self.node.index());
            }
        }

        Ok(())
    }
}

/// Detaches a node from the parent and siblings.
pub(crate) struct Detach {
    node: NodeIdx,
}

impl Detach {
    pub(crate) fn new(node: NodeIdx) -> Self {
        Self { node }
    }
}

impl<T> Relate<T> for Detach {
    fn bind(self, tree: &mut Tree<T>) -> Result<(), RelationError> {
        // Get the node indexes
        let node = tree.node_mut(self.node).ok_or(RelationError::Invalid)?;
        let parent = node.parent.take();
        let next_sibling = node.prev_sibling.take();
        let prev_sibling = node.prev_sibling.take();

        // Link the prev and next sibling
        Self::link_siblings(tree, next_sibling, prev_sibling);

        let parent = parent.and_then(|parent_idx| tree.nodes.get_mut(parent_idx));
        if let Some(node) = parent {
            // Replace first child with previous sibling
            if node.first_child == Some(self.node.index()) {
                node.first_child = prev_sibling;
            }
            // Replace last child with next sibling
            if node.last_child == Some(self.node.index()) {
                node.last_child = prev_sibling;
            }
        }

        Ok(())
    }
}

/// Relate a node as the last child of the parent
pub(crate) struct AppendChild {
    node: NodeIdx,
    parent: NodeIdx,
}

impl AppendChild {
    pub(crate) fn new(node: NodeIdx, parent: NodeIdx) -> Self {
        Self { node, parent }
    }
}

impl<T> Relate<T> for AppendChild {
    fn bind(self, tree: &mut Tree<T>) -> Result<(), RelationError> {
        // Detach the node before making the new relation
        // This will also make sure the node exists
        Detach::new(self.node).bind(tree)?;

        // Get the node indexes
        let parent = tree.node_mut(self.parent).ok_or(RelationError::Invalid)?;
        let old_last_child = parent.last_child;

        // Update the parent
        parent.last_child = Some(self.node.index());
        if parent.first_child.is_none() {
            parent.first_child = Some(self.node.index());
        }

        Self::link_siblings(tree, Some(self.node.index()), old_last_child);

        Ok(())
    }
}
