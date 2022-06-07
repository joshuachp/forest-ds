#[derive(Debug, Clone)]
pub struct Node<T> {
    pub(crate) parent: Option<usize>,
    pub(crate) first_child: Option<usize>,
    pub(crate) last_child: Option<usize>,
    pub(crate) prev_sibling: Option<usize>,
    pub(crate) next_sibling: Option<usize>,
    pub(crate) value: T,
}
