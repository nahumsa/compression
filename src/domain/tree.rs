use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HuffmanNode<T> {
    pub weight: u8,
    pub char: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<T: Clone + Eq> {
    pub node: HuffmanNode<T>,
    pub weight: u8,
    pub left: Option<Box<Tree<T>>>,
    pub right: Option<Box<Tree<T>>>,
}

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Copy + Eq> Tree<T> {
    pub fn new(node: HuffmanNode<T>, weight: u8) -> Self {
        Tree {
            node,
            weight,
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

    pub fn from_vec(values: Vec<HuffmanNode<T>>) -> Self {
        let (first_element, rest) = values.split_first().unwrap();
        Tree::new(*first_element, first_element.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = Tree::new(
            HuffmanNode {
                weight: 0,
                char: 'a',
            },
            0,
        );

        let expect_node = HuffmanNode {
            weight: 0,
            char: 'a',
        };

        let expect_weight = 0;

        assert_eq!(expect_node, tree.node);
        assert_eq!(expect_weight, tree.weight);
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

        let tree = Tree::new(
            HuffmanNode {
                weight: 0,
                char: 'a',
            },
            0,
        )
        .left(Tree::new(
            HuffmanNode {
                weight: 0,
                char: 'b',
            },
            1,
        ));

        if let Some(node) = tree.left {
            assert_eq!(expect_left, node.node);
            assert_eq!(1, node.weight);
        }

        assert_eq!(expect_root, tree.node);
        assert_eq!(0, tree.weight);
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

        let tree = Tree::new(
            HuffmanNode {
                weight: 0,
                char: 'a',
            },
            0,
        )
        .right(Tree::new(
            HuffmanNode {
                weight: 0,
                char: 'b',
            },
            1,
        ));

        if let Some(node) = tree.right {
            assert_eq!(expect_right, node.node);
            assert_eq!(1, node.weight);
        }

        assert_eq!(expect_root, tree.node);
        assert_eq!(0, tree.weight);
    }
    //TODO: add test to insert
}
