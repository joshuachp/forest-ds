//! Arena allocator that backs the Tree structure.

/// Arena allocator for [`Entry`], free entries will be kept and reused.
///
// TODO: statistics on occupied an free entries
#[derive(Debug, Clone)]
pub(crate) struct Store<T> {
    data: Vec<Entry<T>>,
    first_free: Option<usize>,
}

impl<T> Store<T> {
    /// Create a new Store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            first_free: None,
            data: Vec::new(),
        }
    }

    /// Create a new Store with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            first_free: None,
            data: Vec::with_capacity(capacity),
        }
    }

    /// Allocate a value in the arena.
    ///
    /// It will use the first free node or append it at the end if there isn't a free one.
    pub fn allocate(&mut self, value: T) -> usize {
        match self.first_free {
            Some(index) => {
                debug_assert!(index < self.data.len());
                debug_assert!(matches!(self.data[index], Entry::Free { .. }));

                let entry = self.data.get_mut(index).expect("allocated node");
                let old = entry.replace(value);

                self.first_free = old.expect_free();

                index
            }
            None => {
                let index = self.data.len();

                self.data.push(Entry::Occupied(value));

                index
            }
        }
    }

    /// Free the node index
    pub fn free_node(&mut self, index: usize) -> Option<T> {
        self.data.get_mut(index).and_then(|entry| {
            let next_free = self.first_free.replace(index);
            entry.take(next_free).some()
        })
    }

    /// Gets a reference to a node given the index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(Entry::as_ref)
    }

    /// Gets a mutable reference node given the index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index).and_then(Entry::as_mut)
    }
}

impl<U> FromIterator<U> for Store<U> {
    fn from_iter<T: IntoIterator<Item = U>>(iter: T) -> Self {
        let data = iter.into_iter().map(Entry::Occupied).collect();

        Self {
            data,
            first_free: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Entry<T> {
    Free { next_free: Option<usize> },
    Occupied(T),
}

impl<T> Entry<T> {
    pub fn take(&mut self, next_free: Option<usize>) -> Self {
        std::mem::replace(self, Entry::Free { next_free })
    }

    pub fn replace(&mut self, val: T) -> Self {
        std::mem::replace(self, Entry::Occupied(val))
    }

    pub fn is_occupied(&self) -> bool {
        matches!(self, Entry::Occupied(..))
    }

    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Entry::Free { .. } => None,
            Entry::Occupied(value) => Some(value),
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Entry::Free { .. } => None,
            Entry::Occupied(value) => Some(value),
        }
    }

    pub fn some(self) -> Option<T> {
        match self {
            Entry::Free { .. } => todo!(),
            Entry::Occupied(value) => Some(value),
        }
    }

    pub fn expect(self) -> T {
        match self {
            Entry::Free { .. } => panic!("the entry is free"),
            Entry::Occupied(value) => value,
        }
    }

    pub fn expect_ref(&self) -> &T {
        match self {
            Entry::Free { .. } => panic!("the entry is free"),
            Entry::Occupied(value) => value,
        }
    }

    pub fn expect_mut(&mut self) -> &mut T {
        match self {
            Entry::Free { .. } => panic!("the entry is free"),
            Entry::Occupied(value) => value,
        }
    }

    pub fn expect_free(&self) -> Option<usize> {
        match self {
            Entry::Free { next_free } => *next_free,
            Entry::Occupied(_) => panic!("the entry is occupied"),
        }
    }

    pub fn map_ref<'a, U, F>(&'a self, f: F) -> Option<U>
    where
        F: FnOnce(&'a T) -> U,
    {
        match self {
            Entry::Free { .. } => None,
            Entry::Occupied(value) => Some(f(value)),
        }
    }

    pub fn map_mut<'a, U, F>(&'a mut self, f: F) -> Option<U>
    where
        F: FnOnce(&'a mut T) -> U,
    {
        match self {
            Entry::Free { .. } => None,
            Entry::Occupied(value) => Some(f(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_allocate() {
        let mut store = Store::with_capacity(2);

        assert_eq!(store.first_free, None);

        let idx = store.allocate(42);

        assert_eq!(idx, 0);

        let idx2 = store.allocate(1);

        assert_eq!(idx2, 1);

        assert_eq!(store.free_node(idx), Some(42));

        assert_eq!(store.first_free, Some(0));

        assert_eq!(store.allocate(42), 0);

        assert_eq!(store.data.len(), 2);
        assert_eq!(store.data.capacity(), 2);
    }
}
