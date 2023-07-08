use std::{collections::{HashMap, BinaryHeap}, cmp::{Ordering}};

type ChildNode = Option<Box<Node>>;
type HuffMap = HashMap<char, Vec<u8>>;

#[derive(PartialEq, Eq, Debug)]
struct Node {
    left: ChildNode,
    right: ChildNode,
    letter: Option<char>,
    count: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const INPUT: &str = "aaaaaaaaaabcccccccccccccccddddddd";

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

fn gen_huff_tree(mut letters: HashMap<char, usize>) -> Box<Node> {
    let mut heap: BinaryHeap<_> = letters
        .drain()
        .map(|(letter, count)| Box::new(Node {
            letter: Some(letter),
            count,
            left: None,
            right: None
        }))
        .collect();

    while heap.len() > 1 {
        let n1 = heap.pop().unwrap();
        let n2 = heap.pop().unwrap();

        let sum_count = n1.count + n2.count;

        heap.push(Box::new(Node {
            left: Some(n1),
            right: Some(n2),
            letter: None,
            count: sum_count,
        }));
    }

    assert_eq!(heap.len(), 1);
    heap.pop().unwrap()
}

fn print_tree(root: &Node, branch: &str, depth: usize) {

    println!(
        "{}{}( {} | {} )", "│  ".repeat(depth),
        branch,
        root.letter.unwrap_or(' '),
        root.count
    );

    let child_branch = if root.right.is_none() { "└──" } else { "├──" };
    if let Some(nl) = &root.left { print_tree(&nl, child_branch, depth+1) }
    if let Some(nr) = &root.right { print_tree(&nr, child_branch, depth+1) }
}

fn huff_encode(root: &Node) -> HuffMap {
    let mut map: HashMap<char, Vec<u8>> = HashMap::new();
    get_huff_codes(root, vec![], &mut map);
    map
}

fn get_huff_codes(n: &Node, bits: Vec<u8>, map: &mut HuffMap) {
    // invariant: a leaf node will always have Some letter and None for children
    if n.letter.is_some() && n.left.is_none() && n.right.is_none() {
        map.insert(n.letter.clone().unwrap(), bits);
    }
    else {
        let mut bits_l = bits.clone();
        bits_l.push(0);
        get_huff_codes(n.left.as_ref().unwrap(), bits_l, map);
    
        let mut bits_r = bits.clone();
        bits_r.push(1);
        get_huff_codes(n.right.as_ref().unwrap(), bits_r, map);
    }
}

fn ilog2(mut n: usize) -> usize {
    let mut i = 0;
    while n > 1 {
        i += 1;
        n /= 2;
    }
    i
}

fn get_bitlen(n: usize) -> usize {
    ilog2(n) + 1
}

// returns canonical Huffman codes for a letter -> huffcode map.
// see: https://en.wikipedia.org/wiki/Canonical_Huffman_code
fn huff_to_canon(map: HuffMap) -> Vec<(char, usize)> {
    // sort key-value tuples by bitlength first, then letter. ascending order
    let mut bitlens: Vec<_> = map.iter().map(|(k,v)| (k,v.len())).collect();
    bitlens.sort_by(|a,b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    // set first letter's canonical Huffcode to 0
    let mut seq: usize = 0;
    let mut final_bitlens = vec![(*bitlens[0].0, 0)];

    // increment code each time, ensuring its bitlength is as long as its non-canon bitlength
    for (ch, len) in bitlens.iter().skip(1)  {
        seq = (seq + 1) << (len - get_bitlen(seq));
       final_bitlens.push((**ch, seq));
    }

    final_bitlens
}

// TODO implement
fn encode_msg(msg: &str, map: &HuffMap) -> String {
    let mut encoded_msg = String::new();
    for ch in msg.chars() {
        let bits: String = map[&ch]
            .iter()
            .map(|b| std::char::from_digit((*b).into(), 10).unwrap())
            .collect();
        encoded_msg.push_str(&bits);
    }
    encoded_msg
}
fn decode_msg(msg: String, huff_tree: &Node) -> String { "".to_string() }
fn canon_to_huff(codes: Vec<(char, usize)>) -> HuffMap { HuffMap::new() }

fn main() {
    // TODO add file input & command line input
    println!("Input string: \"{}\"", INPUT);
    
    let letters = gen_freq_map(INPUT);
    println!("Frequency map: {:?}", letters);

    let huff_tree = gen_huff_tree(letters);
    print_tree(&huff_tree, "", 0);

    let map = huff_encode(&huff_tree);
    println!("{:?}", map);
    
    let encoded_msg = encode_msg(INPUT, &map);
    println!("Encoded message: {}", encoded_msg);
    
    let canon_codes = huff_to_canon(map);
    println!("{:?}", canon_codes);

}
