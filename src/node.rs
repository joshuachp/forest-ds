#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Node<T> {
    pub(crate) value: T,

    pub(crate) parent: Option<usize>,
    pub(crate) prev_sibling: Option<usize>,
    pub(crate) next_sibling: Option<usize>,
    pub(crate) first_child: Option<usize>,
    pub(crate) last_child: Option<usize>,
}

impl<T> Node<T> {
    pub(crate) const fn new(value: T) -> Self {
        Self {
            value,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
        }
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            parent: Option::default(),
            prev_sibling: Option::default(),
            next_sibling: Option::default(),
            first_child: Option::default(),
            last_child: Option::default(),
        }
    }
}
