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

        self.relate(child_index, Some(parent_index), last_child, None);

        if Some(parent_index) == self.last_node {
            let mut cursor = self.cursor(child).unwrap();

            while cursor.move_next().is_ok() {}

            let id = &cursor.id();
            self.last_node = self.index(id);
        }

        Ok(())
    }

    /// Make the `node` as the previous sibling of `sibling`
    ///
    /// # Errors
    ///
    /// - Fails of the same `NodeId` is passed
    /// - TODO: Fail if the child node is parent of the parent node.
    pub fn make_prev_siblings(&mut self, node: &NodeId, sibling: &NodeId) -> Result<(), Error> {
        let node_index = self.index(node).ok_or(Error::Invalid("for node"))?;
        let sibling_index = self.index(sibling).ok_or(Error::Invalid("for sibling"))?;

        if node_index == sibling_index {
            return Err(Error::SameNode);
        }

        // TODO: search if the child has the parent as child

        let sibling_node = self.nodes[sibling_index].unwrap_ref();
        let parent_index = sibling_node.parent;
        let prev_sibling = sibling_node.prev_sibling;

        self.relate(node_index, parent_index, prev_sibling, Some(sibling_index));

        if Some(sibling_index) == self.first_node {
            self.first_node = Some(node_index);
        }

        Ok(())
    }

    /// Make the `node` as the next sibling of `sibling`
    ///
    /// # Errors
    ///
    /// - Fails of the same `NodeId` is passed
    /// - TODO: Fail if the child node is parent of the parent node.
    pub fn make_next_siblings(&mut self, node: &NodeId, sibling: &NodeId) -> Result<(), Error> {
        let node_index = self.index(node).ok_or(Error::Invalid("for node"))?;
        let sibling_index = self.index(sibling).ok_or(Error::Invalid("for sibling"))?;

        if node_index == sibling_index {
            return Err(Error::SameNode);
        }

        // TODO: search if the child has the parent as child

        let sibling_node = self.nodes[sibling_index].unwrap_ref();
        let parent_index = sibling_node.parent;
        let next_sibling = sibling_node.next_sibling;

        self.relate(node_index, parent_index, Some(sibling_index), next_sibling);

        if Some(sibling_index) == self.last_node {
            self.last_node = Some(node_index);
        }

        Ok(())
    }

    /// Detach the node from it's parent
    ///
    /// # Errors
    ///
    /// - Fails of the `node` was removed
    pub fn detach(&mut self, node: &NodeId) -> Result<(), Error> {
        let node_index = self.index(node).ok_or(Error::Invalid("for node"))?;
        let node = self.nodes[node_index].unwrap_mut();

        let parent = node.parent;
        let prev_sibling = node.prev_sibling;
        let next_sibling = node.next_sibling;

        node.parent = None;

        if let Some(parent_index) = parent {
            let parent = self.nodes[parent_index].unwrap_mut();

            if parent.last_child == Some(node_index) {
                parent.last_child = prev_sibling;
            }

            if parent.first_child == Some(node_index) {
                parent.last_child = next_sibling;
            }
        }

        if let Some(next_sibling_index) = next_sibling {
            let next = self.nodes[next_sibling_index].unwrap_mut();

            next.prev_sibling = prev_sibling;
        }

        if let Some(prev_sibling_index) = prev_sibling {
            let prev = self.nodes[prev_sibling_index].unwrap_mut();

            prev.next_sibling = next_sibling;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::tree::Tree;

    #[test]
    fn should_update_last_node_make_child() {
        let mut tree = Tree::new();

        let parent = tree.append_child(1);

        let node = tree.create_node(2);

        assert_eq!(Some(parent), tree.first_node_id());
        assert_eq!(Some(parent), tree.last_node_id());

        tree.make_child(&node, &parent).unwrap();

        assert_eq!(Some(parent), tree.first_node_id());
        assert_eq!(Some(node), tree.last_node_id());
    }

    #[test]
    fn should_update_first_node_make_sibling() {
        let mut tree = Tree::new();

        let sibling = tree.append_child(1);

        let node = tree.create_node(2);

        assert_eq!(Some(sibling), tree.first_node_id());
        assert_eq!(Some(sibling), tree.last_node_id());

        tree.make_prev_siblings(&node, &sibling).unwrap();

        assert_eq!(Some(node), tree.first_node_id());
        assert_eq!(Some(sibling), tree.last_node_id());
    }

    #[test]
    fn should_update_last_node_make_sibling() {
        let mut tree = Tree::new();

        tree.append_child(1);
        let sibling = tree.append_child(2);

        let node = tree.create_node(3);

        assert_eq!(Some(sibling), tree.last_node_id());

        tree.make_next_siblings(&node, &sibling).unwrap();

        assert_eq!(Some(node), tree.last_node_id());
    }
}
