use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug)]
pub struct HuffManNode {
    freq: usize,
    val: Option<char>,
    right: Option<Box<HuffManNode>>,
    left: Option<Box<HuffManNode>>,
}

impl Ord for HuffManNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for HuffManNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffManNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for HuffManNode {}

pub fn build_frequency(content: String) -> HashMap<char, usize> {
    let mut freq_table = HashMap::new();

    for c in content.chars() {
        let count = freq_table.entry(c).or_insert(0);
        *count += 1;
    }

    freq_table
}

pub fn build_huffman_tree(chars: HashMap<char, usize>) -> Option<HuffManNode> {
    let mut min_heap = BinaryHeap::new();

    for (val, freq) in chars {
        let huffman_node = HuffManNode {
            freq,
            val: Some(val),
            right: None,
            left: None,
        };

        min_heap.push(huffman_node);
    }

    while min_heap.len() > 1 {
        let left = min_heap.pop().unwrap();
        let right = min_heap.pop().unwrap();

        let merged_node = HuffManNode {
            freq: left.freq + right.freq,
            val: None,
            right: Some(Box::new(right)),
            left: Some(Box::new(left)),
        };

        min_heap.push(merged_node);
    }

    min_heap.pop()
}
