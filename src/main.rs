use std::{collections::{HashMap, BinaryHeap}, cmp::{Ordering, Reverse}};

type ChildNode = Option<Box<Node>>;

#[derive(PartialEq, Eq, Debug)]
struct Node {
    left: ChildNode,
    right: ChildNode,
    letter: char,
    count: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const INPUT: &str = "hello world";

fn gen_freq_map(input: &str) -> HashMap<char, usize> {
    let mut letters = HashMap::new();

    for c in String::from(input).chars() {
        letters
            .entry(c)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    letters
}

fn main() {
    // TODO add file input & command line input
    println!("Input string: \"{}\"", INPUT);
    
    let mut letters = gen_freq_map(INPUT);
    println!("Frequency map: {:?}", letters);

    let mut heap: BinaryHeap<_> = letters
        .drain()
        .map(|(letter, count)| Reverse(Node {
            letter,
            count,
            left: None,
            right: None
        }))
        .collect();

    while !heap.is_empty() {
        println!("{:?}", heap.pop())
    }
}
