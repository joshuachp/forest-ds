//! Errors returned by the operations on the [`crate::tree::Tree`].

use crate::relation::RelationError;

/// Errors returned by the tree operations.
#[derive(Debug, Clone, Copy, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// couldn't relate nodes
    Relation(#[from] RelationError),
}
