#[derive(Debug)]
pub struct NodeId {
    pub(crate) index: usize,
    // TODO: we could have a mutable reference to the tree or create an entry struct to be sure the
    // given node index is still valid for removes etc.
}
