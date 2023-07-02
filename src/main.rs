use std::collections::HashMap;

type ChildNode = Option<Box<Node>>;

struct Node {
    left: ChildNode,
    right: ChildNode,
    letter: char,
    count: usize,
}

const INPUT: &str = "hello world";

fn main() {
    let mut letters: HashMap<char, usize> = HashMap::new();

    for c in String::from(INPUT).chars() {
        letters.entry(c)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    println!("{:?}", letters);
}
