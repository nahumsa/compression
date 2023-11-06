use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HuffmanNode {
    pub weight: u8,
    pub char: char,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tree<T> {
    pub node: T,
    pub left: Option<Box<Tree<T>>>,
    pub right: Option<Box<Tree<T>>>,
}

impl<T> Tree<T> {
    pub fn new(node: T) -> Self {
        Tree {
            node: node,
            node,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: Tree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Tree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = Tree::new(HuffmanNode {
            weight: 0,
            char: 'a',
        });
        let expect = HuffmanNode {
            weight: 0,
            char: 'a',
        };

        assert_eq!(expect, tree.node);
    }

    #[test]
    fn test_insert_left() {
        let expect_root = HuffmanNode {
            weight: 0,
            char: 'a',
        };

        let expect_left = HuffmanNode {
            weight: 0,
            char: 'b',
        };

        let tree = Tree::new(HuffmanNode {
            weight: 0,
            char: 'a',
        })
        .left(Tree::new(HuffmanNode {
            weight: 0,
            char: 'b',
        }));

        if let Some(node) = tree.left {
            assert_eq!(expect_left, node.node);
        }

        assert_eq!(expect_root, tree.node);
    }

    #[test]
    fn test_insert_right() {
        let expect_root = HuffmanNode {
            weight: 0,
            char: 'a',
        };

        let expect_right = HuffmanNode {
            weight: 0,
            char: 'b',
        };

        let tree = Tree::new(HuffmanNode {
            weight: 0,
            char: 'a',
        })
        .right(Tree::new(HuffmanNode {
            weight: 0,
            char: 'b',
        }));

        if let Some(node) = tree.right {
            assert_eq!(expect_right, node.node);
        }

        assert_eq!(expect_root, tree.node);
    }
}
