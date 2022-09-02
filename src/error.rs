#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    // Index
    #[error("invalid node id {0}")]
    Invalid(&'static str),
    // Relations
    #[error("same node id provided")]
    SameNode,
}
