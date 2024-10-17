use std::{collections::HashMap, env, fs::File, io::Read};

use compression::{build_frequency, build_huffman_codes, build_huffman_tree};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("file path not provided");

    let mut content = String::new();
    let mut file = File::open(filename).expect("could not open file");
    file.read_to_string(&mut content)
        .expect("could not read file");

    let chars = build_frequency(content);
    let huffman_tree = build_huffman_tree(chars).expect("file is empty");

    let mut huffman_codes = HashMap::new();

    println!("{:#?}", huffman_tree);

    build_huffman_codes(
        &mut huffman_codes,
        &Some(Box::new(huffman_tree)),
        String::new(),
    );

    println!("{:#?}", huffman_codes);
}
