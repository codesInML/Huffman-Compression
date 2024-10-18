use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufReader, Read},
};

use compression::{
    build_frequency, build_huffman_codes, build_huffman_tree, decode_and_write_file,
    read_compressed_file, write_compressed_file,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file path not provided");
    let output_file = "compressed.huf";

    let file = File::open(filename).expect("could not open file");
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .expect("could not read file");

    let chars = build_frequency(&content);
    let huffman_tree = build_huffman_tree(chars).expect("file is empty");

    let mut huffman_codes = HashMap::new();
    let tree = huffman_tree.clone();

    build_huffman_codes(
        &mut huffman_codes,
        &Some(Box::new(huffman_tree)),
        String::new(),
    );

    // println!("{:?}", tree);

    let encoded_data: String = content
        .chars()
        .map(|ch| huffman_codes[&ch].clone())
        .collect();

    write_compressed_file(&tree, &encoded_data, output_file).expect("could not write to file");
    let (built_tree, bit_stream) =
        read_compressed_file(output_file).expect("could not read compressed file");

    decode_and_write_file(&built_tree, &bit_stream, "output.txt").expect("could not decode file");

    // println!("{:?}", built_tree);
    // println!("{:?}", bit_stream);
}
