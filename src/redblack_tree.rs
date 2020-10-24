use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

#[derive(Debug)]
struct Node<T: Ord> {
    key: T,
    lhs: RefCell<Option<Rc<Node<T>>>>,
    rhs: RefCell<Option<Rc<Node<T>>>>,
    parent: RefCell<Option<Weak<Node<T>>>>,
}

impl<T: Ord> Node<T> {
    fn new(key: T, lhs: Option<Node<T>>, rhs: Option<Node<T>>) -> Self {
        Self {
            key,
            lhs: RefCell::new(lhs.map(|node| Rc::new(node))),
            rhs: RefCell::new(rhs.map(|node| Rc::new(node))),
            parent: RefCell::new(None),
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

    fn rotate_right(node: Rc<Node<T>>) -> Rc<Node<T>> {
        if node.lhs.borrow().is_none() {
            return node;
        }

        let new_rotation_root = Rc::clone(node.lhs.borrow().as_ref().unwrap());
        *node.lhs.borrow_mut() = new_rotation_root
            .rhs
            .borrow()
            .as_ref()
            .and_then(|node| Some(Rc::clone(node)));
        *new_rotation_root.rhs.borrow_mut() = Some(node);
        new_rotation_root
    }
}

impl<T: Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key) && self.lhs.eq(&other.lhs) && self.rhs.eq(&other.rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_root() {
        let mut tree = RedBlackTree::new();
        tree.insert(42);
        assert_eq!(
            RedBlackTree {
                root: Some(Rc::new(Node {
                    key: 42,
                    lhs: RefCell::new(None),
                    rhs: RefCell::new(None),
                    parent: RefCell::new(None),
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

    #[test]
    fn right_rotation() {
        let mut tree = RedBlackTree {
            root: Some(Rc::new(Node::new(
                18,
                Some(Node::new(
                    11,
                    Some(Node::new(9, None, None)),
                    Some(Node::new(14, None, None)),
                )),
                Some(Node::new(19, None, None)),
            ))),
        };
        tree.root = Some(Node::rotate_right(tree.root.unwrap()));
        assert_eq!(
            RedBlackTree {
                root: Some(Rc::new(Node::new(
                    11,
                    Some(Node::new(9, None, None)),
                    Some(Node::new(
                        18,
                        Some(Node::new(14, None, None)),
                        Some(Node::new(19, None, None)),
                    )),
                ))),
            },
            tree
        )
    }
}
