#[derive(Debug, Clone)]
pub struct Tree<K, V> {
    root: Option<usize>,
    nodes: Vec<Node<K, V>>,
}

#[derive(Debug, Clone)]
pub enum Node<K, V> {
    Root(Root<K, V>),
    Internal(Internal<K, V>),
    Leaf(Leaf<K, V>),
}

#[derive(Debug, Clone)]
pub struct Root<K, V> {
    key: K,
    value: V,
    child: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Leaf<K, V> {
    key: K,
    value: V,
    parent: usize,
}

#[derive(Debug, Clone)]
pub struct Internal<K, V> {
    key: K,
    value: V,
    parent: usize,
    child: Option<usize>,
    prev_sibling: Option<usize>,
    next_sibling: Option<usize>,
}

impl<K, V> Tree<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            nodes: Vec::new(),
        }
    }
}
