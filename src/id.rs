use crate::{entry::Entry, node::Node, tree::Tree};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId {
    index: usize,
}

impl NodeId {
    pub(crate) fn new(index: usize) -> Self {
        Self { index }
    }

    #[must_use]
    pub(crate) fn eq_index(&self, index: usize) -> bool {
        self.index == index
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

    #[must_use]
    pub(crate) fn get_node(&self, id: &NodeId) -> Option<&Node<T>> {
        match self.nodes.get(id.index) {
            Some(Entry::Occupied(node)) => Some(node),
            _ => None,
        }
    }

    #[must_use]
    pub(crate) fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut Node<T>> {
        match self.nodes.get_mut(id.index) {
            Some(Entry::Occupied(node)) => Some(node),
            _ => None,
        }
    }
}
