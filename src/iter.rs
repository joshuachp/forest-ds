use crate::{node::Node, tree::Tree};

impl<T> Tree<T> {
    #[must_use]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.root,
            nodes: &self.nodes,
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

#[derive(Debug)]
pub struct Iter<'a, T> {
    current: Option<usize>,
    nodes: &'a [Node<T>],
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
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

            &node.value
        })
    }
}

#[cfg(test)]
mod test {
    use crate::tree::Tree;

    #[test]
    fn should_return_none_on_empty() {
        let tree = Tree::<i32>::new();

        assert_eq!(None, tree.iter().next());
    }

    #[test]
    fn should_inter_children() {
        let mut tree = Tree::<i32>::new();

        tree.append_child(1);
        tree.append_child(2);

        let mut iter = tree.iter();

        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
    }

    #[test]
    fn should_inter_siblings() {
        let mut tree = Tree::<i32>::new();

        tree.append_sibling(1);
        tree.append_sibling(2);

        dbg!(&tree);

        let mut iter = tree.iter();

        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
    }
}
