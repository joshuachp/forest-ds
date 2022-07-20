use crate::{entry::Entry, error::Error, id::NodeId, node::Node};

#[derive(Debug, Clone)]
pub struct Tree<T> {
    pub(crate) first_free: Option<usize>,
    pub(crate) first_node: Option<usize>,
    pub(crate) last_node: Option<usize>,
    pub(crate) nodes: Vec<Entry<T>>,
}

impl<T> Tree<T> {
    /// Create a new tree.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new tree with a specific parameter.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            first_free: None,
            first_node: None,
            last_node: None,
            nodes: Vec::with_capacity(capacity),
        }
    }

    /// Add a node to the tree, without relations with the other nodes.
    pub fn create_node(&mut self, value: T) -> NodeId {
        let index = self.allocate_node(Node::new(value));

        NodeId { index }
    }

    pub fn remove(&mut self, id: NodeId) -> Option<T> {
        self.index(&id).map(|index| {
            let entry = self.free_node(index);

            entry.unwrap().value
        })
    }

    #[must_use]
    pub fn first_node_id(&self) -> Option<NodeId> {
        self.first_node.map(|index| NodeId { index })
    }

    #[must_use]
    pub fn last_node_id(&self) -> Option<NodeId> {
        self.first_node.map(|index| NodeId { index })
    }

    /// Insert the last child for a given index.
    pub(crate) fn insert_child_at(&mut self, index: usize, value: T) -> usize {
        let mut node = Node::new(value);
        let node_index = self.get_first_free();

        node.parent = Some(index);

        if Some(index) == self.last_node {
            self.last_node = Some(node_index);
        }

        let parent = self.nodes[index].unwrap_mut();

        let last_child = parent.last_child.replace(node_index);

        match last_child {
            Some(sibling_index) => {
                let sibling = self.nodes[sibling_index].unwrap_mut();
                sibling.next_sibling = Some(node_index);
                node.prev_sibling = Some(sibling_index);
            }
            None => {
                parent.first_child = Some(node_index);
                parent.last_child = Some(node_index);
            }
        }

        let index = self.allocate_node(node);
        debug_assert_eq!(index, node_index);

        node_index
    }

    /// Insert the next sibling for a given index.
    pub(crate) fn insert_sibling_at(&mut self, index: usize, value: T) -> usize {
        let node_index = self.get_first_free();
        let mut node = Node::new(value);

        node.prev_sibling = Some(index);

        let sibling = self.nodes[index].unwrap_mut();
        let next_sibling = sibling.next_sibling.replace(node_index);

        node.next_sibling = next_sibling;
        node.parent = sibling.parent;

        if node.next_sibling.is_none() {
            if let Some(parent) = node.parent {
                self.nodes[parent].unwrap_mut().last_child = Some(node_index);
            }
        }

        if Some(index) == self.last_node {
            self.last_node = Some(node_index);
        }

        let index = self.allocate_node(node);
        debug_assert_eq!(index, node_index);

        node_index
    }

    /// Appends the value to the last element of the three as its child. If None creates a new root.
    pub fn append_child(&mut self, value: T) -> NodeId {
        let index = match self.last_node {
            Some(tail_index) => self.insert_child_at(tail_index, value),
            None => {
                let index = self.allocate_node(Node::new(value));
                self.first_node = Some(index);
                self.last_node = Some(index);

                index
            }
        };

        NodeId { index }
    }

    /// Appends the value to the last element of the three as its sibling. If None creates a new
    /// root.
    pub fn append_sibling(&mut self, value: T) -> NodeId {
        let index = match self.last_node {
            Some(tail_index) => self.insert_sibling_at(tail_index, value),
            None => {
                let index = self.allocate_node(Node::new(value));
                self.first_node = Some(index);
                self.last_node = Some(index);

                index
            }
        };

        NodeId { index }
    }

    pub fn append_child_to(&mut self, id: &NodeId, value: T) -> Result<NodeId, Error> {
        let index = self.index(id).ok_or(Error::Invalid("passed"))?;

        let index = self.insert_child_at(index, value);

        Ok(NodeId { index })
    }

    pub fn insert_sibling_after(&mut self, id: &NodeId, value: T) -> Result<NodeId, Error> {
        let index = self.index(id).ok_or(Error::Invalid("passed"))?;

        let index = self.insert_sibling_at(index, value);

        Ok(NodeId { index })
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            first_free: Option::default(),
            first_node: Option::default(),
            last_node: Option::default(),
            nodes: Vec::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{entry::Entry, node::Node};
    use pretty_assertions::assert_eq;

    use super::Tree;

    #[test]
    pub fn should_create_root_on_append_child() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_child(42);

        assert_eq!(Some(0), tree.first_node);
        assert_eq!(Some(0), tree.last_node);

        let node = Node {
            value: 42,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(Entry::Occupied(node), tree.nodes[0]);
    }

    #[test]
    pub fn should_append_child() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_child(1);
        tree.append_child(2);

        assert_eq!(Some(0), tree.first_node);
        assert_eq!(Some(1), tree.last_node);

        let first = Node {
            value: 1,
            parent: None,
            first_child: Some(1),
            last_child: Some(1),
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(Entry::Occupied(first), tree.nodes[0]);

        let second = Node {
            value: 2,
            parent: Some(0),
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(Entry::Occupied(second), tree.nodes[1]);
    }

    #[test]
    pub fn should_create_root_on_append_sibling() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_sibling(42);

        assert_eq!(Some(0), tree.first_node);
        assert_eq!(Some(0), tree.last_node);

        let node = Node {
            value: 42,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(Entry::Occupied(node), tree.nodes[0]);
    }

    #[test]
    pub fn should_append_sibling() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_sibling(1);
        tree.append_sibling(2);

        assert_eq!(Some(0), tree.first_node);
        assert_eq!(Some(1), tree.last_node);

        let first = Node {
            value: 1,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: Some(1),
            prev_sibling: None,
        };

        assert_eq!(Entry::Occupied(first), tree.nodes[0]);

        let second = Node {
            value: 2,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: Some(0),
        };

        assert_eq!(Entry::Occupied(second), tree.nodes[1]);
    }

    #[test]
    fn should_append_child_and_siblings() {
        let mut tree: Tree<i32> = Tree::new();

        let root = tree.append_child(0);

        let first = tree.append_child_to(&root, 1).unwrap();
        tree.insert_sibling_after(&first, 2).unwrap();

        let root = Node {
            value: 0,
            parent: None,
            first_child: Some(1),
            last_child: Some(2),
            next_sibling: None,
            prev_sibling: None,
        };
        assert_eq!(Entry::Occupied(root), tree.nodes[0]);

        let first = Node {
            value: 1,
            parent: Some(0),
            first_child: None,
            last_child: None,
            next_sibling: Some(2),
            prev_sibling: None,
        };
        assert_eq!(Entry::Occupied(first), tree.nodes[1]);

        let second = Node {
            value: 2,
            parent: Some(0),
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: Some(1),
        };
        assert_eq!(Entry::Occupied(second), tree.nodes[2]);
    }
}
