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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let expect = Tree {
            node: HuffmanNode {
                value: 0,
                char: 'a',
            },
            left: None,
            right: None,
        };
        let got = Tree::new(HuffmanNode {
            value: 0,
            char: 'a',
        });
        assert_eq!(expect, got);
    }
}
