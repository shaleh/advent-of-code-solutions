use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct Node<'a> {
    low: Option<&'a str>,
    high: Option<&'a str>,
    values: Vec<i64>,
}

impl<'a> Node<'a> {
    fn new_with_paths(low: &'a str, high: &'a str) -> Self {
        Self {
            low: Some(low),
            high: Some(high),
            ..Default::default()
        }
    }

    fn new_with_value(value: i64) -> Self {
        Self {
            values: vec![value],
            ..Default::default()
        }
    }
}

fn main() {
    let data = include_str!("input");
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for line in data.lines() {
        let pieces: Vec<&str> = line.trim().split(" ").collect();
        println!("{:?}", pieces);
        match pieces[0] {
            "value" => {
                let value = pieces[1].parse().unwrap();
                let entry = nodes
                    .entry(pieces[5])
                    .and_modify(|node| {
                        node.values.push(value);
                        node.values.sort();
                        node.values.reverse();
                    })
                    .or_insert(Node::new_with_value(value));
                queue.push(entry.clone());
            }
            "bot" => {
                nodes
                    .entry(pieces[1])
                    .and_modify(|node| {
                        node.low = Some(pieces[6]);
                        node.high = Some(pieces[11]);
                    })
                    .or_insert(Node::new_with_paths(pieces[6], pieces[11]));
            }
            otherwise => panic!("{}", otherwise),
        }
    }

    println!("{:?}", nodes);
    println!("{:?}", queue);

    loop {
        let node = queue.pop();
        dbg!(node);
        break;
        if queue.is_empty() {
            break;
        }
    }
}
