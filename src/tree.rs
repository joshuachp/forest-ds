#[derive(Debug, Clone)]
pub struct Tree<V> {
    root: Option<usize>,
    tail: Option<usize>,
    nodes: Vec<Node<V>>,
}

#[derive(Debug, Clone)]
pub struct Node<V> {
    parent: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,
    prev_sibling: Option<usize>,
    next_sibling: Option<usize>,
    value: V,
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

        let root_index = Some(self.nodes.len());
        self.root = root_index;
        self.tail = root_index;

        self.nodes.push(root);
    }

    fn set_sibling(&mut self, index: usize, value: V) -> usize {
        let mut node = Node::new(value);
        let node_index = self.nodes.len();

        let sibling = &mut self.nodes[index];
        sibling.next_sibling = Some(node_index);

        node.prev_sibling = Some(index);
        node.parent = sibling.parent;

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

impl<V> Node<V> {
    pub const fn new(value: V) -> Self {
        Self {
            parent: None,
            first_child: None,
            last_child: None,
            prev_sibling: None,
            next_sibling: None,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, V> {
    index: Option<usize>,
    nodes: &'a [Node<V>],
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
    index: Option<usize>,
    nodes: Vec<Option<Node<V>>>,
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
    index: Option<usize>,
    nodes: Vec<Option<&'a mut Node<V>>>,
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
