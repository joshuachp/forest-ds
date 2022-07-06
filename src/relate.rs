//! Creates and changes relationships between nodes

use crate::{error::Error, id::NodeId, tree::Tree};

impl<T> Tree<T> {
    /// Makes three nodes related to each other
    fn relate(
        &mut self,
        node_index: usize,
        parent_index: Option<usize>,
        prev_sibling_index: Option<usize>,
        next_sibling_index: Option<usize>,
    ) {
        let node = &mut self.nodes[node_index];
        node.parent = parent_index;
        node.prev_sibling = prev_sibling_index;
        node.next_sibling = next_sibling_index;

        // If the parent doesn't have children set the node as first and last
        if let Some(parent_index) = parent_index {
            let parent = &mut self.nodes[parent_index];

            if prev_sibling_index.is_none() {
                parent.first_child = Some(node_index);
            }

            if next_sibling_index.is_none() {
                parent.last_child = Some(node_index);
            }
        }

        if let Some(prev_sibling_index) = prev_sibling_index {
            let prev_sibling = &mut self.nodes[prev_sibling_index];

            debug_assert!(prev_sibling.next_sibling.is_none());
            debug_assert_eq!(prev_sibling.parent, parent_index);

            prev_sibling.next_sibling = Some(node_index);
        }

        if let Some(next_sibling_index) = next_sibling_index {
            let next_sibling = &mut self.nodes[next_sibling_index];

            debug_assert!(next_sibling.prev_sibling.is_none());
            debug_assert_eq!(next_sibling.parent, parent_index);

            next_sibling.prev_sibling = Some(node_index);
        }
    }

    /// Make the `child` nodes as the last child of the `parent` node.
    ///
    /// # Errors
    ///
    /// - Fails of the same `NodeId` is passed
    /// - TODO: Fail if the child node is parent of the parent node.
    pub fn make_child(&mut self, child: &NodeId, parent: &NodeId) -> Result<(), Error> {
        debug_assert!(child.index < self.nodes.len());
        debug_assert!(parent.index < self.nodes.len());

        if child.eq(parent) {
            return Err(Error::SameNode);
        }

        // TODO: search if the child has the parent as child

        let parent_node = &self.nodes[parent.index];
        let last_child = parent_node.last_child;

        self.relate(child.index, Some(parent.index), last_child, None);

        Ok(())
    }
}
