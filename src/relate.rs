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
        let node = self.nodes[node_index].unwrap_mut();
        node.parent = parent_index;
        node.prev_sibling = prev_sibling_index;
        node.next_sibling = next_sibling_index;

        // If the parent doesn't have children set the node as first and last
        if let Some(parent_index) = parent_index {
            let parent = self.nodes[parent_index].unwrap_mut();

            if prev_sibling_index.is_none() {
                parent.first_child = Some(node_index);
            }

            if next_sibling_index.is_none() {
                parent.last_child = Some(node_index);
            }
        }

        if let Some(prev_sibling_index) = prev_sibling_index {
            let prev_sibling = self.nodes[prev_sibling_index].unwrap_mut();

            debug_assert!(prev_sibling.next_sibling.is_none());
            debug_assert_eq!(prev_sibling.parent, parent_index);

            prev_sibling.next_sibling = Some(node_index);
        }

        if let Some(next_sibling_index) = next_sibling_index {
            let next_sibling = self.nodes[next_sibling_index].unwrap_mut();

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
        let child_index = self.index(child).ok_or(Error::Invalid("for child"))?;
        let parent_index = self.index(parent).ok_or(Error::Invalid("for parent"))?;

        if child_index == parent_index {
            return Err(Error::SameNode);
        }

        // TODO: search if the child has the parent as child

        let parent_node = self.nodes[parent_index].unwrap_ref();
        let last_child = parent_node.last_child;

        self.relate(child_index, Some(parent.index), last_child, None);

        Ok(())
    }
}
