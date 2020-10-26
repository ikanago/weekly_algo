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
                self.root = Some(Node::new(key, None, None));
            }
        }
    }
}

#[derive(Debug)]
struct Node<T: Ord> {
    key: T,
    lhs: RefCell<Option<Rc<Node<T>>>>,
    rhs: RefCell<Option<Rc<Node<T>>>>,
    parent: RefCell<Weak<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(key: T, lhs: Option<Rc<Node<T>>>, rhs: Option<Rc<Node<T>>>) -> Rc<Self> {
        let node = Rc::new(Self {
            key,
            lhs: RefCell::new(None),
            rhs: RefCell::new(None),
            parent: RefCell::new(Weak::new()),
        });
        if let Some(lhs) = lhs {
            *lhs.parent.borrow_mut() = Rc::downgrade(&node);
            *node.lhs.borrow_mut() = Some(lhs);
        }
        if let Some(rhs) = rhs {
            *rhs.parent.borrow_mut() = Rc::downgrade(&node);
            *node.rhs.borrow_mut() = Some(rhs);
        }
        node
    }

    fn rotate_left(node: Rc<Node<T>>) -> Rc<Node<T>> {
        if node.rhs.borrow().is_none() {
            return node;
        }

        let new_rotation_root = Rc::clone(node.rhs.borrow().as_ref().unwrap());
        new_rotation_root.parent.swap(&node.parent);
        node.parent.replace(Rc::downgrade(&new_rotation_root));

        *node.rhs.borrow_mut() = new_rotation_root
            .lhs
            .borrow()
            .as_ref()
            .and_then(|node| Some(Rc::clone(node)));
        if let Some(lhs) = new_rotation_root.lhs.borrow().as_ref() {
            lhs.parent.replace(Rc::downgrade(&node));
        }
        *new_rotation_root.lhs.borrow_mut() = Some(node);
        new_rotation_root
    }

    fn rotate_right(node: Rc<Node<T>>) -> Rc<Node<T>> {
        if node.lhs.borrow().is_none() {
            return node;
        }

        let new_rotation_root = Rc::clone(node.lhs.borrow().as_ref().unwrap());
        new_rotation_root.parent.swap(&node.parent);
        node.parent.replace(Rc::downgrade(&new_rotation_root));

        *node.lhs.borrow_mut() = new_rotation_root
            .rhs
            .borrow()
            .as_ref()
            .and_then(|node| Some(Rc::clone(node)));
        if let Some(rhs) = new_rotation_root.rhs.borrow().as_ref() {
            rhs.parent.replace(Rc::downgrade(&node));
        }
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

    fn is_valid<T: Ord>(node: &Rc<Node<T>>) -> bool {
        let is_lhs_valid = if let Some(lhs) = node.lhs.borrow().as_ref() {
            if node.key <= lhs.key || node.key != lhs.parent.borrow().upgrade().unwrap().key {
                false
            } else {
                is_valid(lhs)
            }
        } else {
            true
        };
        let is_rhs_valid = if let Some(rhs) = node.rhs.borrow().as_ref() {
            if node.key >= rhs.key || node.key != rhs.parent.borrow().upgrade().unwrap().key {
                false
            } else {
                is_valid(rhs)
            }
        } else {
            true
        };
        is_lhs_valid && is_rhs_valid
    }

    #[test]
    fn insert_root() {
        let mut tree = RedBlackTree::new();
        tree.insert(42);
        assert!(is_valid(&tree.root.clone().unwrap()));
        assert_eq!(
            RedBlackTree {
                root: Some(Rc::new(Node {
                    key: 42,
                    lhs: RefCell::new(None),
                    rhs: RefCell::new(None),
                    parent: RefCell::new(Weak::new()),
                }))
            },
            tree
        );
    }

    #[test]
    fn left_rotation() {
        let mut node = Node::new(
            11,
            Some(Node::new(9, None, None)),
            Some(Node::new(
                18,
                Some(Node::new(14, None, None)),
                Some(Node::new(19, None, None)),
            )),
        );
        node = Node::rotate_left(node);
        assert!(is_valid(&node));
        assert_eq!(
            Node::new(
                18,
                Some(Node::new(
                    11,
                    Some(Node::new(9, None, None)),
                    Some(Node::new(14, None, None)),
                )),
                Some(Node::new(19, None, None)),
            ),
            node
        )
    }

    #[test]
    fn right_rotation() {
        let mut node = Node::new(
            18,
            Some(Node::new(
                11,
                Some(Node::new(9, None, None)),
                Some(Node::new(14, None, None)),
            )),
            Some(Node::new(19, None, None)),
        );
        node = Node::rotate_right(node);
        assert!(is_valid(&node));
        assert_eq!(
            Node::new(
                11,
                Some(Node::new(9, None, None)),
                Some(Node::new(
                    18,
                    Some(Node::new(14, None, None)),
                    Some(Node::new(19, None, None)),
                )),
            ),
            node
        )
    }
}
