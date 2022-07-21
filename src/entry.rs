use crate::{node::Node, tree::Tree};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Entry<T> {
    Free { next_free: Option<usize> },
    Occupied(Node<T>),
}

impl<T> Entry<T> {
    pub fn replace(&mut self, val: Self) -> Self {
        std::mem::replace(self, val)
    }

    pub fn is_node(&self) -> bool {
        matches!(self, Entry::Occupied(..))
    }

    pub fn unwrap(self) -> Node<T> {
        match self {
            Entry::Free { .. } => panic!("the entry is free"),
            Entry::Occupied(node) => node,
        }
    }

    pub fn unwrap_ref(&self) -> &Node<T> {
        match self {
            Entry::Free { .. } => panic!("the entry is free"),
            Entry::Occupied(node) => node,
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut Node<T> {
        match self {
            Entry::Free { .. } => panic!("the entry is free"),
            Entry::Occupied(node) => node,
        }
    }

    pub fn unwrap_free(&self) -> Option<usize> {
        match self {
            Entry::Free { next_free } => *next_free,
            Entry::Occupied(_) => panic!("the entry is occupied"),
        }
    }

    pub fn map_ref<'a, U, F>(&'a self, f: F) -> Option<U>
    where
        F: FnOnce(&'a Node<T>) -> U,
    {
        match self {
            Entry::Free { .. } => None,
            Entry::Occupied(node) => Some(f(node)),
        }
    }

    pub fn map_mut<'a, U, F>(&'a mut self, f: F) -> Option<U>
    where
        F: FnOnce(&'a mut Node<T>) -> U,
    {
        match self {
            Entry::Free { .. } => None,
            Entry::Occupied(node) => Some(f(node)),
        }
    }
}

impl<T> Tree<T> {
    pub(crate) fn get_first_free(&self) -> usize {
        self.first_free.unwrap_or(self.nodes.len())
    }

    pub(crate) fn allocate_node(&mut self, node: Node<T>) -> usize {
        match self.first_free {
            Some(index) => {
                let entry = self.nodes[index].replace(Entry::Occupied(node));

                self.first_free = entry.unwrap_free();

                index
            }
            None => {
                let index = self.nodes.len();

                self.nodes.push(Entry::Occupied(node));

                index
            }
        }
    }

    pub(super) fn free_node(&mut self, index: usize) -> Entry<T> {
        let next_free = self.first_free.replace(index);

        self.nodes[index].replace(Entry::Free { next_free })
    }
}
