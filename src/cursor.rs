//! Module for the `Entry` API to easily move a reference in the tree.

use crate::{id::NodeId, tree::Tree};

/// Cursor over the tree elements.
///
/// If a move operation fails we `Return` a result with:
/// - `Ok`: the moved cursor
/// - `Err`: the previous unmodified cursor
#[derive(Debug)]
pub struct Cursor<'a, T> {
    index: usize,
    tree: &'a mut Tree<T>,
}

impl<T> Tree<T> {
    pub fn cursor(&mut self, id: &NodeId) -> Option<Cursor<T>> {
        self.index(id).map(|index| Cursor { index, tree: self })
    }

    pub fn cursor_first(&mut self) -> Option<Cursor<T>> {
        self.first_node.map(|index| Cursor { index, tree: self })
    }

    pub fn cursor_last(&mut self) -> Option<Cursor<T>> {
        self.last_node.map(|index| Cursor { index, tree: self })
    }
}

impl<T> Cursor<'_, T> {
    #[must_use]
    pub fn get(&self) -> &T {
        &self.tree.nodes[self.index].unwrap_ref().value
    }

    #[must_use]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.tree.nodes[self.index].unwrap_mut().value
    }

    /// Move the cursor to the parent
    ///
    /// # Errors
    ///
    /// If there is no next node will return the previous position
    pub fn parent(&mut self) -> Result<&mut Self, &mut Self> {
        match self.tree.nodes[self.index].unwrap_ref().parent {
            Some(index) => {
                self.index = index;
                Ok(self)
            }
            None => Err(self),
        }
    }

    /// Move the cursor to the first child
    ///
    /// # Errors
    ///
    /// If there is no next node will return the previous position
    pub fn first_child(&mut self) -> Result<&mut Self, &mut Self> {
        match self.tree.nodes[self.index].unwrap_ref().first_child {
            Some(index) => {
                self.index = index;
                Ok(self)
            }
            None => Err(self),
        }
    }

    /// Move the cursor to the last child
    ///
    /// # Errors
    ///
    /// If there is no next node will return the previous position
    pub fn last_child(&mut self) -> Result<&mut Self, &mut Self> {
        match self.tree.nodes[self.index].unwrap_ref().last_child {
            Some(index) => {
                self.index = index;
                Ok(self)
            }
            None => Err(self),
        }
    }

    /// Move the cursor to the next sibling
    ///
    /// # Errors
    ///
    /// If there is no next node will return the previous position
    pub fn next_sibling(&mut self) -> Result<&mut Self, &mut Self> {
        match self.tree.nodes[self.index].unwrap_ref().next_sibling {
            Some(index) => {
                self.index = index;
                Ok(self)
            }
            None => Err(self),
        }
    }

    /// Move the cursor to the prev sibling
    ///
    /// # Errors
    ///
    /// If there is no next node will return the previous position
    pub fn prev_sibling(&mut self) -> Result<&mut Self, &mut Self> {
        match self.tree.nodes[self.index].unwrap_ref().prev_sibling {
            Some(index) => {
                self.index = index;
                Ok(self)
            }
            None => Err(self),
        }
    }

    /// Move the cursor to the next node
    ///
    /// # Errors
    ///
    /// If there is no next node will return the previous position
    pub fn move_next(&mut self) -> Result<&mut Self, &mut Self> {
        self.first_child()
            .or_else(Cursor::next_sibling)
            .or_else(|cursor| {
                let mut parent = &cursor.tree.nodes[cursor.index].unwrap_ref().parent;

                // Iterate to each parent to check if one has a next sibling
                while let Some(parent_index) = parent {
                    let node = cursor.tree.nodes[*parent_index].unwrap_ref();

                    if let Some(sibling) = node.next_sibling {
                        cursor.index = sibling;

                        return Ok(cursor);
                    }

                    parent = &node.parent;
                }

                Err(cursor)
            })
    }

    pub fn append_child(&mut self, value: T) -> NodeId {
        let index = self.tree.insert_child_at(self.index, value);

        NodeId::new(index)
    }

    pub fn append_sibling(&mut self, value: T) -> NodeId {
        let index = self.tree.insert_sibling_at(self.index, value);

        NodeId::new(index)
    }
}

#[cfg(test)]
mod test {
    use crate::tree::Tree;

    #[test]
    fn should_move_next() {
        let mut tree: Tree<i32> = Tree::new();
        // A
        tree.append_child(0);

        // A -> B
        let b = tree.append_child(1);

        // A -> B -> C
        tree.append_child(2);

        // A -> B -> D
        //   -> C
        tree.insert_sibling_after(&b, 3).unwrap();

        let mut cursor = tree.cursor_first().unwrap();

        assert_eq!(0, *cursor.get());

        cursor.move_next().unwrap();
        assert_eq!(1, *cursor.get());

        cursor.move_next().unwrap();
        assert_eq!(2, *cursor.get());

        cursor.move_next().unwrap();
        assert_eq!(3, *cursor.get());

        assert!(cursor.move_next().is_err());
    }
}
