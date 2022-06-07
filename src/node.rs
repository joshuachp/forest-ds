#[derive(Debug, Clone)]
pub struct Node<T> {
    pub(crate) parent: Option<usize>,
    pub(crate) first_child: Option<usize>,
    pub(crate) last_child: Option<usize>,
    pub(crate) prev_sibling: Option<usize>,
    pub(crate) next_sibling: Option<usize>,
    pub(crate) value: T,
}

impl<V> Node<V> {
    pub(crate) const fn new(value: V) -> Self {
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
