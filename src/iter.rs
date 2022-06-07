use crate::{node::Node, tree::Tree};

#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    pub(crate) index: Option<usize>,
    pub(crate) nodes: &'a [Node<T>],
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        let node = &self.nodes[index];

        self.index = if let Some(child) = node.first_child {
            Some(child)
        } else if let Some(next) = node.next_sibling {
            Some(next)
        } else if let Some(parent) = node.parent {
            self.nodes[parent].next_sibling
        } else {
            None
        };

        Some(&node.value)
    }
}

#[derive(Debug, Clone)]
pub struct IntoIter<V> {
    pub(crate) index: Option<usize>,
    pub(crate) nodes: Vec<Option<Node<V>>>,
}

impl<V> Iterator for IntoIter<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        self.nodes[index].take().map(|node| {
            if let Some(child) = node.first_child {
                self.index = Some(child);
            } else if let Some(next) = node.next_sibling {
                self.index = Some(next);
            } else if let Some(parent) = node.parent {
                self.index = self.nodes[parent]
                    .as_ref()
                    .and_then(|parent| parent.next_sibling);
            } else {
                self.index = None;
            }

            node.value
        })
    }
}

#[derive(Debug)]
pub struct IterMut<'a, V> {
    pub(crate) index: Option<usize>,
    pub(crate) nodes: Vec<Option<&'a mut Node<V>>>,
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index?;
        self.nodes[index].take().map(|node| {
            if let Some(child) = node.first_child {
                self.index = Some(child);
            } else if let Some(next) = node.next_sibling {
                self.index = Some(next);
            } else if let Some(parent) = node.parent {
                self.index = self.nodes[parent]
                    .as_ref()
                    .and_then(|parent| parent.next_sibling);
            } else {
                self.index = None;
            }

            &mut node.value
        })
    }
}

impl<'a, V> IntoIterator for &'a Tree<V> {
    type Item = &'a V;

    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<V> IntoIterator for Tree<V> {
    type Item = V;

    type IntoIter = IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iterator()
    }
}

impl<'a, V> IntoIterator for &'a mut Tree<V> {
    type Item = &'a mut V;

    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
