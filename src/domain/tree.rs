use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct HuffmanNode {
    pub value: u8,
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
            value: 0,
            char: 'a',
        });
        let expect = HuffmanNode {
            value: 0,
            char: 'a',
        };

        assert_eq!(expect, tree.node);
    }

    #[test]
    fn test_insert_left() {
        let expect_root = HuffmanNode {
            value: 0,
            char: 'a',
        };

        let expect_left = HuffmanNode {
            value: 0,
            char: 'b',
        };

        let tree = Tree::new(HuffmanNode {
            value: 0,
            char: 'a',
        })
        .left(Tree::new(HuffmanNode {
            value: 0,
            char: 'b',
        }));

        if let Some(node) = tree.left {
            assert_eq!(expect_left, node.node);
        }

        assert_eq!(expect_root, tree.node);
    }
}
