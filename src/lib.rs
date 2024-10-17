use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io::{Error, Write},
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

pub fn build_frequency(content: &String) -> HashMap<char, usize> {
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

pub fn build_huffman_codes(
    huffman_codes: &mut HashMap<char, String>,
    huffman_node: &Option<Box<HuffManNode>>,
    prefix: String,
) {
    if let Some(node) = huffman_node {
        if let Some(val) = node.val {
            huffman_codes.insert(val, prefix);
        } else {
            build_huffman_codes(huffman_codes, &node.left, format!("{}0", prefix));
            build_huffman_codes(huffman_codes, &node.right, format!("{}1", prefix));
        }
    }
}

pub struct BitWriter<W: Write> {
    writer: W,     // The output writer (buffered for performance)
    buffer: u8,    // Accumulated bits (up to 8 bits = 1 byte)
    bit_count: u8, // How many bits are currently in the buffer
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> Self {
        BitWriter {
            writer,
            buffer: 0,
            bit_count: 0,
        }
    }

    pub fn write_bit(&mut self, bit: bool) -> Result<(), Error> {
        self.buffer <<= 1;
        if bit {
            self.buffer |= 1;
        }
        self.bit_count += 1;

        if self.bit_count == 8 {
            self.flush_buffer()?;
        }
        Ok(())
    }

    pub fn flush_buffer(&mut self) -> Result<(), Error> {
        if self.bit_count > 0 {
            self.buffer <<= 8 - self.bit_count;
            self.writer.write_all(&[self.buffer])?;
            self.buffer = 0;
            self.bit_count = 0;
        }
        Ok(())
    }

    pub fn finish(mut self) -> Result<(), Error> {
        self.flush_buffer()
    }
}
