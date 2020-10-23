#[derive(Debug, PartialEq)]
pub struct RedBlackTree<T: Ord> {
    root: Option<Node<T>>,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

#[derive(Debug, PartialEq)]
struct Node<T: Ord> {
    key: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initilization() {
        assert_eq!(RedBlackTree { root: None }, RedBlackTree::<u64>::new());
    }
}
