#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod arena;
pub mod error;
pub mod index;
pub mod node;
mod relation;
#[cfg(feature = "serde")]
pub mod serde;
pub mod tree;

pub use tree::Tree;
