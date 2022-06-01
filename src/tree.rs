#[derive(Debug, Clone)]
pub struct Tree<V> {
    ends: Option<(usize, usize)>,
    nodes: Vec<Node<V>>,
}

#[derive(Debug, Clone)]
pub struct Node<V> {
    parent: Option<usize>,
    child: Option<usize>,
    prev_sibling: Option<usize>,
    next_sibling: Option<usize>,
    value: V,
}

impl<V> Tree<V> {
    pub fn new() -> Self {
        Self {
            ends: None,
            nodes: Vec::new(),
        }
    }

    fn new_root(&mut self, value: V) {
        let root = Node::new(value);

        let root_index = self.nodes.len();
        self.ends = Some((root_index, root_index));

        self.nodes.push(root);
    }

    fn next_sibling(&self, index: usize) -> Option<usize> {
        self.nodes.get(index).and_then(|node| node.next_sibling)
    }

    fn set_sibling(&mut self, index: usize, value: V) -> usize {
        let mut node = Node::new(value);
        let node_index = self.nodes.len();

        let mut sibling_index = index;
        while let Some(next) = self.next_sibling(sibling_index) {
            sibling_index = next;
        }

        let sibling = &mut self.nodes[sibling_index];
        sibling.next_sibling = Some(node_index);

        node.prev_sibling = Some(sibling_index);
        node.parent = sibling.parent;

        node_index
    }

    fn set_child(&mut self, index: usize, value: V) -> usize {
        let child_index = self.nodes.len();
        let parent = &mut self.nodes[index];

        match parent.child {
            Some(sibling) => {
                return self.set_sibling(sibling, value);
            }
            None => {
                let mut child = Node::new(value);

                parent.child = Some(child_index);
                child.parent = Some(index);

                self.nodes.push(child);
            }
        }
        child_index
    }

    pub fn append_child(&mut self, value: V) {
        match self.ends {
            None => self.new_root(value),
            Some((root, tail)) => {
                let child_index = self.set_child(tail, value);

                self.ends = Some((root, child_index));
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<V> {
        let nodes = self.nodes.iter_mut().map(|ref_mut| Some(ref_mut)).collect();

        IterMut {
            index: self.ends.map(|(root, _)| root),
            nodes,
        }
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
    pub fn new(value: V) -> Self {
        Self {
            parent: None,
            child: None,
            prev_sibling: None,
            next_sibling: None,
            value,
        }
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
            if let Some(child) = node.child {
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
            dbg!(&tree);
            tree.append_child(i);
        }
    }

    #[test]
    fn should_iter_mut() {
        let mut tree: Tree<i32> = Tree::new();

        for i in 0..10 {
            dbg!(&tree);
            tree.append_child(i);
        }

        for (i, v) in tree.into_iter().enumerate() {
            assert_eq!(i as i32, *v);
        }
    }
}
