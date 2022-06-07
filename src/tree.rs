use std::fmt::Debug;

use crate::{
    iter::{IntoIter, Iter, IterMut},
    node::Node,
};

#[derive(Debug, Clone)]
pub struct Tree<T> {
    root: Option<usize>,
    tail: Option<usize>,
    nodes: Vec<Node<T>>,
}

impl<V> Tree<V> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            root: None,
            tail: None,
            nodes: Vec::with_capacity(capacity),
        }
    }

    fn new_root(&mut self, value: V) {
        let root = Node::new(value);

        let root_index = self.nodes.len();
        self.root = Some(root_index);
        self.tail = Some(root_index);

        self.nodes.push(root);
    }

    fn set_sibling(&mut self, index: usize, value: V) -> usize {
        let mut node = Node::new(value);
        let node_index = self.nodes.len();

        let sibling = &mut self.nodes[index];
        sibling.next_sibling = Some(node_index);

        node.prev_sibling = Some(index);
        node.parent = sibling.parent;

        match sibling.parent {
            Some(parent) => {
                let mut parent = &mut self.nodes[parent];
                parent.last_child = Some(node_index);
            }
            None => {}
        }

        self.nodes.push(node);

        node_index
    }

    fn set_child(&mut self, index: usize, value: V) -> usize {
        let child_index = self.nodes.len();
        let parent = &mut self.nodes[index];

        match parent.last_child {
            Some(sibling) => self.set_sibling(sibling, value),
            None => {
                let mut child = Node::new(value);

                parent.last_child = Some(child_index);
                child.parent = Some(index);

                self.nodes.push(child);
                child_index
            }
        }
    }

    pub fn append_child(&mut self, value: V) {
        match self.tail {
            None => self.new_root(value),
            Some(tail) => {
                let child_index = self.set_child(tail, value);

                self.tail = Some(child_index);
            }
        }
    }

    pub fn append_sibling(&mut self, value: V) {
        match self.tail {
            None => self.new_root(value),
            Some(tail) => {
                let child_index = self.set_sibling(tail, value);

                self.tail = Some(child_index);
            }
        }
    }

    #[must_use]
    pub fn iter(&self) -> Iter<V> {
        Iter {
            index: self.root,
            nodes: &self.nodes,
        }
    }

    #[must_use]
    pub fn into_iterator(self) -> IntoIter<V> {
        let nodes = self.nodes.into_iter().map(Some).collect();

        IntoIter {
            index: self.root,
            nodes,
        }
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<V> {
        let nodes = self.nodes.iter_mut().map(Some).collect();

        IterMut {
            index: self.root,
            nodes,
        }
    }
}

impl<V> Default for Tree<V> {
    fn default() -> Self {
        Self {
            root: None,
            tail: None,
            nodes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn should_append_child() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..10 {
            tree.append_child(i);
        }
    }

    #[test]
    fn should_append_sibling() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..10 {
            tree.append_sibling(i);
        }
    }

    #[test]
    fn should_iter() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..10 {
            tree.append_child(i);
        }

        for (i, v) in tree.iter().enumerate() {
            assert_eq!(i as i32, *v);
        }
    }

    #[test]
    fn should_into_iter() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..10 {
            tree.append_child(i);
        }

        for (i, v) in tree.into_iterator().enumerate() {
            assert_eq!(i as i32, v);
        }
    }

    #[test]
    fn should_iter_mut() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..10 {
            tree.append_child(i);
        }

        for (i, v) in tree.iter_mut().enumerate() {
            assert_eq!(i as i32, *v);
        }
    }
}
