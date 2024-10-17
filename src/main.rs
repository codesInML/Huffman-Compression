use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufWriter, Read},
};

use compression::{build_frequency, build_huffman_codes, build_huffman_tree, BitWriter};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file path not provided");

    let mut content = String::new();
    let mut file = File::open(filename).expect("could not open file");
    file.read_to_string(&mut content)
        .expect("could not read file");

    let chars = build_frequency(&content);
    let huffman_tree = build_huffman_tree(chars).expect("file is empty");

    let mut huffman_codes = HashMap::new();

    build_huffman_codes(
        &mut huffman_codes,
        &Some(Box::new(huffman_tree)),
        String::new(),
    );

    let encoded: String = content
        .chars()
        .map(|ch| huffman_codes[&ch].clone())
        .collect();

    let file = File::create("compressed.huf").expect("could not create file");
    let mut bit_writer = BitWriter::new(BufWriter::new(file));

    // Write each bit from the encoded string
    for bit in encoded.chars() {
        match bit {
            '1' => bit_writer.write_bit(true).expect("could not write to file"),
            '0' => bit_writer
                .write_bit(false)
                .expect("could not write to file"),
            _ => panic!("Invalid bit in the encoded string: {}", bit),
        }
    }

    // Ensure all remaining bits are flushed
    bit_writer.finish().expect("could not write to file");
}
