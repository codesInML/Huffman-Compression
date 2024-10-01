use std::{collections::HashMap, env, fs::File, io::Read};

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

    let mut chars: HashMap<char, usize> = HashMap::new();

    for c in content.chars() {
        let count = chars.entry(c).or_insert(0);
        *count += 1;
    }

    println!("X occurs {:?} times", chars.get(&'X'));
    println!("t occurs {:?} times", chars.get(&'t'));
}
