//! Index of a node in the tree.

/// Index to a node that is accessible in a [`Tree`].
#[derive(Debug, Clone, Copy)]
pub struct NodeIdx {
    index: usize,
}

impl NodeIdx {
    /// Create a new with the given index.
    pub(crate) fn new(index: usize) -> Self {
        Self { index }
    }

    /// Gets the nodes index in the tree.
    #[inline]
    pub(crate) fn index(&self) -> usize {
        self.index
    }
}
