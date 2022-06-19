use crate::{id::NodeId, node::Node};

#[derive(Debug, Clone)]
pub struct Tree<T> {
    pub(crate) root: Option<usize>,
    pub(crate) tail: Option<usize>,
    pub(crate) nodes: Vec<Node<T>>,
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
            root: None,
            tail: None,
            nodes: Vec::with_capacity(capacity),
        }
    }

    /// Insert the last child for a given index.
    fn insert_child_at(&mut self, index: usize, value: T) -> usize {
        let node_index = self.nodes.len();
        let mut node = Node::new(value);
        node.parent = Some(index);

        if Some(index) == self.tail {
            self.tail = Some(node_index);
        }

        let parent = &mut self.nodes[index];

        let last_child = parent.last_child.replace(node_index);

        match last_child {
            Some(sibling_index) => {
                let sibling = &mut self.nodes[sibling_index];
                sibling.next_sibling = Some(node_index);
                node.prev_sibling = Some(sibling_index);
            }
            None => {
                parent.first_child = Some(node_index);
                parent.last_child = Some(node_index);
            }
        }

        self.nodes.push(node);

        node_index
    }

    /// Insert the next sibling for a given index.
    fn insert_sibling_at(&mut self, index: usize, value: T) -> usize {
        let node_index = self.nodes.len();
        let mut node = Node::new(value);
        node.prev_sibling = Some(index);

        let sibling = &mut self.nodes[index];
        let next_sibling = sibling.next_sibling.replace(node_index);

        node.next_sibling = next_sibling;
        node.parent = sibling.parent;

        if node.next_sibling.is_none() {
            if let Some(parent) = node.parent {
                self.nodes[parent].last_child = Some(node_index);
            }
        }

        if Some(index) == self.tail {
            self.tail = Some(node_index);
        }

        self.nodes.push(node);

        node_index
    }

    /// Appends the value to the last element of the three as its child. If None creates a new root.
    pub fn append_child(&mut self, value: T) -> NodeId {
        let index = match self.tail {
            Some(tail_index) => self.insert_child_at(tail_index, value),
            None => {
                // The tree must be empty, since we don't have a tail node. We can just push a new
                // Node and set the root and tail to 0.
                self.nodes.push(Node::new(value));
                self.root = Some(0);
                self.tail = Some(0);

                0
            }
        };

        NodeId { index }
    }

    /// Appends the value to the last element of the three as its sibling. If None creates a new
    /// root.
    pub fn append_sibling(&mut self, value: T) -> NodeId {
        let index = match self.tail {
            Some(tail_index) => self.insert_sibling_at(tail_index, value),
            None => {
                // The tree must be empty, since we don't have a tail node. We can just push a new
                // Node and set the root and tail to 0.
                const ROOT_INDEX: usize = 0;
                self.nodes.push(Node::new(value));
                self.root = Some(ROOT_INDEX);
                self.tail = Some(ROOT_INDEX);

                ROOT_INDEX
            }
        };

        NodeId { index }
    }

    /// Check that a `NodeId` exists.
    #[must_use]
    pub fn check_id(&self, id: &NodeId) -> bool {
        self.nodes.len() > id.index
    }

    #[must_use]
    pub fn append_child_to(&mut self, id: &NodeId, value: T) -> Option<NodeId> {
        if !self.check_id(id) {
            return None;
        }

        let index = self.insert_child_at(id.index, value);

        Some(NodeId { index })
    }

    #[must_use]
    pub fn insert_sibling_after(&mut self, id: &NodeId, value: T) -> Option<NodeId> {
        if !self.check_id(id) {
            return None;
        }

        let index = self.insert_sibling_at(id.index, value);

        Some(NodeId { index })
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            root: Option::default(),
            tail: Option::default(),
            nodes: Vec::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::node::Node;
    use pretty_assertions::assert_eq;

    use super::Tree;

    #[test]
    pub fn should_create_root_on_append_child() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_child(42);

        assert_eq!(Some(0), tree.root);
        assert_eq!(Some(0), tree.tail);

        let node = Node {
            value: 42,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(node, tree.nodes[0]);
    }

    #[test]
    pub fn should_append_child() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_child(1);
        tree.append_child(2);

        assert_eq!(Some(0), tree.root);
        assert_eq!(Some(1), tree.tail);

        let first = Node {
            value: 1,
            parent: None,
            first_child: Some(1),
            last_child: Some(1),
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(first, tree.nodes[0]);

        let second = Node {
            value: 2,
            parent: Some(0),
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(second, tree.nodes[1]);
    }

    #[test]
    pub fn should_create_root_on_append_sibling() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_sibling(42);

        assert_eq!(Some(0), tree.root);
        assert_eq!(Some(0), tree.tail);

        let node = Node {
            value: 42,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: None,
        };

        assert_eq!(node, tree.nodes[0]);
    }

    #[test]
    pub fn should_append_sibling() {
        let mut tree: Tree<i32> = Tree::new();
        tree.append_sibling(1);
        tree.append_sibling(2);

        assert_eq!(Some(0), tree.root);
        assert_eq!(Some(1), tree.tail);

        let first = Node {
            value: 1,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: Some(1),
            prev_sibling: None,
        };

        assert_eq!(first, tree.nodes[0]);

        let second = Node {
            value: 2,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: Some(0),
        };

        assert_eq!(second, tree.nodes[1]);
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
        assert_eq!(root, tree.nodes[0]);

        let first = Node {
            value: 1,
            parent: Some(0),
            first_child: None,
            last_child: None,
            next_sibling: Some(2),
            prev_sibling: None,
        };
        assert_eq!(first, tree.nodes[1]);

        let second = Node {
            value: 2,
            parent: Some(0),
            first_child: None,
            last_child: None,
            next_sibling: None,
            prev_sibling: Some(1),
        };
        assert_eq!(second, tree.nodes[2]);
    }
}
