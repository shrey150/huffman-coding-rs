use std::collections::HashMap;

type ChildNode = Option<Box<Node>>;

struct Node {
    left: ChildNode,
    right: ChildNode,
    letter: char,
    count: usize,
}

const INPUT: &str = "hello world";

fn gen_freq_map(input: &str) -> HashMap<char, usize> {
    let mut letters = HashMap::new();

    for c in String::from(input).chars() {
        letters.entry(c)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    letters
}

fn main() {
    // TODO add file input & command line input
    println!("Input string: \"{}\"", INPUT);
    
    let letters = gen_freq_map(INPUT);
    println!("Frequency map: {:?}", letters);
}
