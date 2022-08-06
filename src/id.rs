use crate::tree::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId {
    index: usize,
}

impl NodeId {
    pub(crate) fn new(index: usize) -> Self {
        Self { index }
    }
}

impl<T> Tree<T> {
    #[must_use]
    pub fn get(&self, id: &NodeId) -> Option<&T> {
        self.nodes.get(id.index)?.map_ref(|node| &node.value)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &NodeId) -> Option<&mut T> {
        self.nodes
            .get_mut(id.index)?
            .map_mut(|node| &mut node.value)
    }

    #[must_use]
    pub(crate) fn index(&self, id: &NodeId) -> Option<usize> {
        self.nodes.get(id.index).and_then(|entry| {
            if entry.is_node() {
                Some(id.index)
            } else {
                None
            }
        })
    }
}
