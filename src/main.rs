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
        map.entry(n.letter.clone().unwrap()).or_insert(bits);
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
fn to_canonical_huff(old_map: HuffMap) -> HashMap<char, usize> {
    let mut bitlens: Vec<_> = old_map.iter().map(|(k,v)| (k,v.len())).collect();
    bitlens.sort_by(|a,b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    println!("{:?}", bitlens);

    let mut seq: usize = 0;
    let mut map: HashMap<char, usize> = HashMap::new();
    map.insert(*bitlens[0].0, 0);

    for (ch, len) in bitlens.iter().skip(1)  {
        // increment seq, ensuring its bitlength equals its non-canon bitlength (len)
        seq = (seq + 1) << (len - get_bitlen(seq));
        map.insert(**ch, seq);
    }

    map
}

fn main() {
    // TODO add file input & command line input
    println!("Input string: \"{}\"", INPUT);
    
    let letters = gen_freq_map(INPUT);
    println!("Frequency map: {:?}", letters);

    let huff_tree = gen_huff_tree(letters);
    print_tree(&huff_tree, "", 0);

    let map = huff_encode(&huff_tree);
    println!("{:?}", map);

    let canon_map = to_canonical_huff(map);
    println!("{:?}", canon_map);
}
