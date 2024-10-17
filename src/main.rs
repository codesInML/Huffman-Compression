use std::{env, fs::File, io::Read};

use compression::{build_frequency, build_huffman_tree};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    let file;
    match filename {
        Some(name) => file = name,
        None => panic!("file path not provided"),
    }

    let mut content = String::new();
    let mut file = File::open(file).expect("could not open file");
    file.read_to_string(&mut content)
        .expect("could not read file");

    let chars = build_frequency(content);
    let huffman_node = build_huffman_tree(chars);
    let huffman_tree;
    match huffman_node {
        Some(node) => huffman_tree = node,
        None => panic!("file is empty"),
    }

    println!("{:?}", huffman_tree);
}
