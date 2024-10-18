use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufReader, BufWriter, Error, Read, Write},
};

pub enum Mode {
    Compress,
    Decompress,
}

pub struct Config {
    mode: Mode,
    input_file: String,
    output_file: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 4 {
            return Err("Not enough arguments passed".to_string());
        }

        let mode = args[1].clone();

        if mode != "C" && mode != "D" {
            return Err("Invalid argument. Mode can only be C or D".to_string());
        }

        let mode = if mode == "C" {
            Mode::Compress
        } else {
            Mode::Decompress
        };

        let config = Config {
            mode,
            input_file: args[2].clone(),
            output_file: args[3].clone(),
        };

        Ok(config)
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn get_input_file(&self) -> &str {
        &self.input_file
    }

    pub fn get_output_file(&self) -> &str {
        &self.output_file
    }
}

#[derive(Debug, Clone)]
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

pub fn read_file(filename: &str) -> Result<String, Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

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

pub fn write_compressed_file(
    root: &HuffManNode,
    encoded_data: &str,
    output_file: &str,
) -> Result<(), Error> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    let mut tree_string = String::new();
    serialize_tree(root, &mut tree_string);

    // Write the length of the serialized tree as a header
    writer.write_all(&(tree_string.len() as u32).to_le_bytes())?;

    // Write the serialized tree to the file
    writer.write_all(tree_string.as_bytes())?;

    // Write the compressed bit stream
    let mut bit_buffer = 0u8;
    let mut bit_count = 0;

    for bit in encoded_data.chars() {
        bit_buffer <<= 1;
        if bit == '1' {
            bit_buffer |= 1;
        }
        bit_count += 1;

        if bit_count == 8 {
            writer.write_all(&[bit_buffer])?;
            bit_buffer = 0;
            bit_count = 0;
        }
    }

    // Write any remaining bits
    if bit_count > 0 {
        bit_buffer <<= 8 - bit_count;
        writer.write_all(&[bit_buffer])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn serialize_tree(node: &HuffManNode, result: &mut String) {
    if let Some(c) = node.val {
        // 'L' indicates a leaf node
        result.push('L');
        result.push(c);
    } else {
        // 'I' indicates an internal node
        result.push('I');
        if let Some(left) = &node.left {
            serialize_tree(left, result);
        }
        if let Some(right) = &node.right {
            serialize_tree(right, result);
        }
    }
}

pub fn deserialize_tree<I>(chars: &mut I) -> Option<Box<HuffManNode>>
where
    I: Iterator<Item = char>,
{
    match chars.next() {
        Some('L') => {
            let c = chars.next().unwrap();
            Some(Box::new(HuffManNode {
                freq: 0,
                val: Some(c),
                left: None,
                right: None,
            }))
        }
        Some('I') => {
            let left = deserialize_tree(chars);
            let right = deserialize_tree(chars);
            Some(Box::new(HuffManNode {
                freq: 0,
                val: None,
                left,
                right,
            }))
        }
        _ => None,
    }
}

pub fn read_compressed_file(input_file: &str) -> Result<(Box<HuffManNode>, Vec<u8>), Error> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);

    let mut length_buffer = [0u8; 4];
    reader.read_exact(&mut length_buffer)?;
    let tree_length = u32::from_le_bytes(length_buffer);

    let mut tree_string = vec![0u8; tree_length as usize];
    reader.read_exact(&mut tree_string)?;
    let tree_string = String::from_utf8(tree_string).unwrap();

    let tree = deserialize_tree(&mut tree_string.chars());

    let mut bit_stream = Vec::new();
    reader.read_to_end(&mut bit_stream)?;

    Ok((tree.unwrap(), bit_stream))
}

pub fn decode_and_write_file(
    root: &HuffManNode,
    bit_stream: &[u8],
    output_file: &str,
) -> Result<(), Error> {
    let mut writer = BufWriter::new(File::create(output_file)?);
    let mut current_node = root;

    let mut buffer = Vec::new();

    for byte in bit_stream {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;

            current_node = if bit == 0 {
                current_node.left.as_deref().unwrap()
            } else {
                current_node.right.as_deref().unwrap()
            };

            if let Some(c) = current_node.val {
                buffer.push(c as u8);
                current_node = root;
            }
        }
    }

    writer.write_all(&buffer)?;
    writer.flush()?;

    Ok(())
}
