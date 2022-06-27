#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum Error {
    // Relations
    #[error("same node id provided")]
    SameNode,
}
