use crate::tree::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId {
    pub(crate) index: usize,
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

    pub(crate) fn index(&self, id: &NodeId) -> Option<usize> {
        if self.nodes.len() > id.index {
            Some(id.index)
        } else {
            None
        }
    }
}
