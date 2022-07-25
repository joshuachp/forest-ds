/// Implements iteration over a tree
use crate::{entry::Entry, id::NodeId, node::Node, tree::Tree};

impl<T> Tree<T> {
    #[must_use]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.first_node,
            nodes: &self.nodes,
        }
    }

    #[must_use]
    pub fn iter_from(&self, id: &NodeId) -> Iter<T> {
        Iter {
            current: self.index(id),
            nodes: &self.nodes,
        }
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            current: self.first_node,
            nodes: self
                .nodes
                .iter_mut()
                .filter_map(|entry| match entry {
                    Entry::Free { .. } => None,
                    Entry::Occupied(node) => Some(Node {
                        value: Some(&mut node.value),
                        parent: node.parent,
                        first_child: node.first_child,
                        last_child: node.last_child,
                        next_sibling: node.next_sibling,
                        prev_sibling: node.prev_sibling,
                    }),
                })
                .collect(),
        }
    }

    #[must_use]
    pub fn into_iterator(self) -> IntoIter<T> {
        IntoIter {
            current: self.first_node,
            nodes: self
                .nodes
                .into_iter()
                .filter_map(|entry| match entry {
                    Entry::Free { .. } => None,
                    Entry::Occupied(node) => Some(Node {
                        value: Some(node.value),
                        parent: node.parent,
                        first_child: node.first_child,
                        last_child: node.last_child,
                        next_sibling: node.next_sibling,
                        prev_sibling: node.prev_sibling,
                    }),
                })
                .collect(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Tree<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> IntoIterator for Tree<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iterator()
    }
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    current: Option<usize>,
    nodes: &'a [Entry<T>],
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let node = &self.nodes[current];

            if let Some(child) = node.unwrap_ref().first_child {
                self.current = Some(child);
            } else if let Some(sibling) = node.unwrap_ref().next_sibling {
                self.current = Some(sibling);
            } else {
                // Start from the current node
                let mut next = node.unwrap_ref();
                // Cycle to the parent to search for the next sibling or go up the tree
                while let Some(parent_index) = next.parent {
                    next = self.nodes[parent_index].unwrap_ref();
                    if next.next_sibling.is_some() {
                        break;
                    }
                }
                // If next.sibling is Some we have the next node, otherwise both next.parent is None
                // and next.next_sibling is None
                self.current = next.next_sibling;
            }

            &node.unwrap_ref().value
        })
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T> {
    current: Option<usize>,
    nodes: Vec<Node<Option<&'a mut T>>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().and_then(|current| {
            // TODO: this could done more safely
            let node = &self.nodes[current];

            if let Some(child) = node.first_child {
                self.current = Some(child);
            } else if let Some(sibling) = node.next_sibling {
                self.current = Some(sibling);
            } else {
                // Start from the current node
                let mut next = node;
                // Cycle to the parent to search for the next sibling or go up the tree
                while let Some(parent_index) = next.parent {
                    next = &self.nodes[parent_index];
                    if next.next_sibling.is_some() {
                        break;
                    }
                }
                // If next.sibling is Some we have the next node, otherwise both next.parent is None
                // and next.next_sibling is None
                self.current = next.next_sibling;
            }

            self.nodes[current].value.take()
        })
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    current: Option<usize>,
    nodes: Vec<Node<Option<T>>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().and_then(|current| {
            let node = &self.nodes[current];

            if let Some(child) = node.first_child {
                self.current = Some(child);
            } else if let Some(sibling) = node.next_sibling {
                self.current = Some(sibling);
            } else {
                // Start from the current node
                let mut next = node;
                // Cycle to the parent to search for the next sibling or go up the tree
                while let Some(parent_index) = next.parent {
                    next = &self.nodes[parent_index];
                    if next.next_sibling.is_some() {
                        break;
                    }
                }
                // If next.sibling is Some we have the next node, otherwise both next.parent is None
                // and next.next_sibling is None
                self.current = next.next_sibling;
            }

            self.nodes[current].value.take()
        })
    }
}

#[cfg(test)]
mod test {
    use crate::tree::Tree;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_return_none_on_empty() {
        let tree = Tree::<i32>::new();

        assert_eq!(None, tree.iter().next());
    }

    #[test]
    fn should_iter_children() {
        let mut tree = Tree::<i32>::new();

        tree.append_child(1);
        tree.append_child(2);

        let mut iter = tree.iter();

        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
    }

    #[test]
    fn should_iter_siblings() {
        let mut tree = Tree::<i32>::new();

        tree.append_sibling(1);
        tree.append_sibling(2);

        dbg!(&tree);

        let mut iter = tree.iter();

        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
    }

    #[test]
    fn mut_should_return_none_on_empty() {
        let mut tree = Tree::<i32>::new();

        assert_eq!(None, tree.iter_mut().next());
    }

    #[test]
    fn mut_should_iter_children() {
        let mut tree = Tree::<i32>::new();

        tree.append_child(1);
        tree.append_child(2);

        let mut iter = tree.iter_mut();

        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
    }

    #[test]
    fn mut_should_iter_siblings() {
        let mut tree = Tree::<i32>::new();

        tree.append_sibling(1);
        tree.append_sibling(2);

        dbg!(&tree);

        let mut iter = tree.iter_mut();

        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
    }

    #[test]
    fn into_should_return_none_on_empty() {
        let tree = Tree::<i32>::new();

        assert_eq!(None, tree.into_iterator().next());
    }

    #[test]
    fn into_should_iter_children() {
        let mut tree = Tree::<i32>::new();

        tree.append_child(1);
        tree.append_child(2);

        let mut iter = tree.into_iterator();

        assert_eq!(1, iter.next().unwrap());
        assert_eq!(2, iter.next().unwrap());
    }

    #[test]
    fn into_should_iter_siblings() {
        let mut tree = Tree::<i32>::new();

        tree.append_sibling(1);
        tree.append_sibling(2);

        dbg!(&tree);

        let mut iter = tree.into_iterator();

        assert_eq!(1, iter.next().unwrap());
        assert_eq!(2, iter.next().unwrap());
    }
}
