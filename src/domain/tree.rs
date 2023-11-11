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

    fn from_freq_table(table: HashMap<T, u64>) -> Self {
        let mut heap = BinaryHeap::new();

        for (char, weight) in table {
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
        table.insert('c', 32);
        table.insert('d', 42);
        table.insert('e', 120);
        table.insert('k', 7);
        table.insert('l', 42);
        table.insert('m', 24);
        table.insert('u', 37);
        table.insert('z', 2);
        let got = HuffmanNode::from_freq_table(table);

        assert_eq!(got.weight, 306);
    }
}
