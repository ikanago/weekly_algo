use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct RedBlackTree<T: Ord> {
    root: Option<Rc<Node<T>>>,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: T) {
        match &self.root {
            Some(_) => unimplemented!(),
            None => {
                self.root = Some(Rc::new(Node::new(key, None, None)));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node<T: Ord> {
    key: T,
    lhs: RefCell<Option<Rc<Node<T>>>>,
    rhs: RefCell<Option<Rc<Node<T>>>>,
}

impl<T: Ord> Node<T> {
    fn new(key: T, lhs: Option<Node<T>>, rhs: Option<Node<T>>) -> Self {
        Self {
            key,
            lhs: RefCell::new(lhs.map(|node| Rc::new(node))),
            rhs: RefCell::new(rhs.map(|node| Rc::new(node))),
        }
    }

    fn rotate_left(node: Rc<Node<T>>) -> Rc<Node<T>> {
        if node.rhs.borrow().is_none() {
            return node;
        }

        let new_rotation_root = Rc::clone(node.rhs.borrow().as_ref().unwrap());
        *node.rhs.borrow_mut() = new_rotation_root
            .lhs
            .borrow()
            .as_ref()
            .and_then(|node| Some(Rc::clone(node)));
        *new_rotation_root.lhs.borrow_mut() = Some(node);
        new_rotation_root
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
                root: Some(Rc::new(Node {
                    key: 42,
                    lhs: RefCell::new(None),
                    rhs: RefCell::new(None)
                }))
            },
            tree
        );
    }

    #[test]
    fn left_rotation() {
        let mut tree = RedBlackTree {
            root: Some(Rc::new(Node::new(
                11,
                Some(Node::new(9, None, None)),
                Some(Node::new(
                    18,
                    Some(Node::new(14, None, None)),
                    Some(Node::new(19, None, None)),
                )),
            ))),
        };
        tree.root = Some(Node::rotate_left(tree.root.unwrap()));
        assert_eq!(
            RedBlackTree {
                root: Some(Rc::new(Node::new(
                    18,
                    Some(Node::new(
                        11,
                        Some(Node::new(9, None, None)),
                        Some(Node::new(14, None, None)),
                    )),
                    Some(Node::new(19, None, None)),
                ))),
            },
            tree
        )
    }
}
