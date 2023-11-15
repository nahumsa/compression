use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HuffmanNode<T> {
    pub weight: u64,
    pub char: Option<T>,
    pub left: Option<Box<HuffmanNode<T>>>,
    pub right: Option<Box<HuffmanNode<T>>>,
}

impl<T: Clone + Eq> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: Clone + Eq> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + Eq> HuffmanNode<T> {
    pub fn leaf(weight: u64, char: Option<T>) -> Self {
        Self {
            weight,
            char,
            left: None,
            right: None,
        }
    }

    pub fn node(weight: u64, left: HuffmanNode<T>, right: HuffmanNode<T>) -> Self {
        Self {
            weight,
            char: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn from_freq_table(table: HashMap<T, u64>) -> Self {
        let mut heap = BinaryHeap::new();

        for (char, weight) in table {
            // TODO: fix this for same weight, when this occurs, the response is non deterministic
            heap.push(Reverse(HuffmanNode::leaf(weight, Some(char))));
        }

        while heap.len() > 1 {
            let node_1 = heap.pop().unwrap().0;
            let node_2 = heap.pop().unwrap().0;

            let merge_node = HuffmanNode::node(node_1.weight + node_2.weight, node_1, node_2);
            heap.push(Reverse(merge_node));
        }

        heap.pop().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huffman_tree_correct() {
        let mut table = HashMap::new();
        table.insert('a', 10);
        table.insert('b', 20);
        table.insert('c', 110);
        table.insert('d', 80);

        let got = HuffmanNode::from_freq_table(table);

        assert_eq!(got.weight, 220);
        assert_eq!(got.char, None);

        assert_eq!(got.left.clone().unwrap().weight, 110);
        assert_eq!(got.left.clone().unwrap().char, Some('c'));

        assert_eq!(got.right.clone().unwrap().weight, 110);
        assert_eq!(got.right.clone().unwrap().char, None);

        assert_eq!(got.right.clone().unwrap().right.clone().unwrap().weight, 80);
        assert_eq!(
            got.right.clone().unwrap().right.clone().unwrap().char,
            Some('d')
        );

        assert_eq!(got.right.clone().unwrap().left.clone().unwrap().weight, 30);
        assert_eq!(got.right.clone().unwrap().left.clone().unwrap().char, None);

        assert_eq!(
            got.right
                .clone()
                .unwrap()
                .left
                .clone()
                .unwrap()
                .left
                .clone()
                .unwrap()
                .weight,
            10
        );
        assert_eq!(
            got.right
                .clone()
                .unwrap()
                .left
                .clone()
                .unwrap()
                .left
                .clone()
                .unwrap()
                .char,
            Some('a')
        );

        assert_eq!(
            got.right
                .clone()
                .unwrap()
                .left
                .clone()
                .unwrap()
                .right
                .clone()
                .unwrap()
                .weight,
            20
        );
        assert_eq!(
            got.right
                .clone()
                .unwrap()
                .left
                .clone()
                .unwrap()
                .right
                .clone()
                .unwrap()
                .char,
            Some('b')
        );
    }
}
