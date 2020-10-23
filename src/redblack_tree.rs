#[derive(Debug, PartialEq)]
pub struct RedBlackTree<T: Ord> {
    root: Option<Node<T>>,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: T) {
        match &self.root {
            Some(_) => unimplemented!(),
            None => {
                self.root = Some(Node::new(key));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node<T: Ord> {
    key: T,
}

impl<T: Ord> Node<T> {
    fn new(key: T) -> Self {
        Self { key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initilization() {
        assert_eq!(RedBlackTree { root: None }, RedBlackTree::<u64>::new());
    }

    #[test]
    fn insert_root() {
        let mut tree = RedBlackTree::new();
        tree.insert(42);
        assert_eq!(
            RedBlackTree {
                root: Some(Node::new(42))
            },
            tree
        );
    }
}
