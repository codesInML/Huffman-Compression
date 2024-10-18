use std::{collections::HashMap, env};

use compression::{
    build_frequency, build_huffman_codes, build_huffman_tree, decode_and_write_file,
    read_compressed_file, read_file, write_compressed_file, Config,
    Mode::{Compress, Decompress},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap();

    match config.get_mode() {
        Compress => {
            let content = read_file(config.get_input_file()).expect("could not read file");

            let chars = build_frequency(&content);
            let huffman_tree = build_huffman_tree(chars).expect("file is empty");

            let mut huffman_codes = HashMap::new();
            let tree = huffman_tree.clone();

            build_huffman_codes(
                &mut huffman_codes,
                &Some(Box::new(huffman_tree)),
                String::new(),
            );

            let encoded_data: String = content
                .chars()
                .map(|ch| huffman_codes[&ch].clone())
                .collect();

            write_compressed_file(
                &tree,
                &encoded_data,
                &format!("{}.huf", config.get_output_file()),
            )
            .expect("could not write to file");
        }
        Decompress => {
            let (built_tree, bit_stream) = read_compressed_file(config.get_input_file())
                .expect("could not read compressed file");
            decode_and_write_file(
                &built_tree,
                &bit_stream,
                &format!("{}.txt", config.get_output_file()),
            )
            .expect("could not decode file");
        }
    }
}
