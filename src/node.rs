//! Node inside the tree.

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use crate::index::NodeIdx;

/// Node value in the [`crate::Tree`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Node<T> {
    pub(crate) value: T,
    pub(crate) parent: Option<usize>,
    pub(crate) prev_sibling: Option<usize>,
    pub(crate) next_sibling: Option<usize>,
    pub(crate) first_child: Option<usize>,
    pub(crate) last_child: Option<usize>,
}

impl<T> Node<T> {
    pub(crate) const fn new(value: T) -> Self {
        Self {
            value,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
        }
    }

    /// Get the index for the parent node.
    pub fn parent(&self) -> Option<NodeIdx> {
        self.parent.map(NodeIdx::new)
    }

    /// Get the index for the previous sibling node.
    pub fn prev_sibling(&self) -> Option<NodeIdx> {
        self.prev_sibling.map(NodeIdx::new)
    }

    /// Get the index for the next sibling node.
    pub fn next_sibling(&self) -> Option<NodeIdx> {
        self.next_sibling.map(NodeIdx::new)
    }

    /// Get the index for the first child node.
    pub fn first_child(&self) -> Option<NodeIdx> {
        self.first_child.map(NodeIdx::new)
    }

    /// Get the index for the last child node.
    pub fn last_child(&self) -> Option<NodeIdx> {
        self.last_child.map(NodeIdx::new)
    }
}

impl<T> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for Node<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Borrow<T> for Node<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<T> BorrowMut<T> for Node<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
