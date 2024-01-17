use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

type Position = (i64, i64);
type Pad = HashMap<Position, char>;
type Rules = HashMap<Position, [i64; 4]>;

fn walk_pad(pad: &Pad, rules: &Rules, start_position: Position, data: &str) {
    for line in data.lines() {
        let mut position = start_position;
        for (k, group) in &line.chars().group_by(|x| *x) {
            let count = group.count() as i64;
            let rules = rules.get(&position).unwrap();
            position = match k {
                'L' => (max(position.0 - count, rules[0]), position.1),
                'U' => (position.0, min(position.1 + count, rules[1])),
                'R' => (min(position.0 + count, rules[2]), position.1),
                'D' => (position.0, max(position.1 - count, rules[3])),
                _ => unreachable!(),
            };
        }
        println!("{:?} {}", position, pad.get(&position).unwrap());
    }
}

fn round1(data: &str) {
    let mut pad: Pad = HashMap::new();
    pad.insert((-1, 1), '1');
    pad.insert((0, 1), '2');
    pad.insert((1, 1), '3');
    pad.insert((-1, 0), '4');
    pad.insert((0, 0), '5');
    pad.insert((1, 0), '6');
    pad.insert((-1, -1), '7');
    pad.insert((0, -1), '8');
    pad.insert((1, -1), '9');
    // [L, U, R, D]
    let mut rule_lookup: Rules = HashMap::new();
    rule_lookup.insert((-1, 1), [-1, 1, 1, -1]);
    rule_lookup.insert((0, 1), [-1, 1, 1, -1]);
    rule_lookup.insert((1, 1), [-1, 1, 1, -1]);
    rule_lookup.insert((-1, 0), [-1, 1, 1, -1]);
    rule_lookup.insert((0, 0), [-1, 1, 1, -1]);
    rule_lookup.insert((1, 0), [-1, 1, 1, -1]);
    rule_lookup.insert((-1, -1), [-1, 1, 1, -1]);
    rule_lookup.insert((0, -1), [-1, 1, 1, -1]);
    rule_lookup.insert((1, -1), [-1, 1, 1, -1]);

    walk_pad(&pad, &rule_lookup, (0, 0), data);
}

fn round2(data: &str) {
    let mut pad: Pad = HashMap::new();
    pad.insert((0, 2), '1');
    pad.insert((-1, 1), '2');
    pad.insert((0, 1), '3');
    pad.insert((1, 1), '4');
    pad.insert((-2, 0), '5');
    pad.insert((-1, 0), '6');
    pad.insert((0, 0), '7');
    pad.insert((1, 0), '8');
    pad.insert((2, 0), '9');
    pad.insert((-1, -1), 'A');
    pad.insert((0, -1), 'B');
    pad.insert((1, -1), 'C');
    pad.insert((0, -2), 'D');

    let mut rule_lookup: Rules = HashMap::new();
    rule_lookup.insert((0, 2), [0, 2, 0, -2]);
    rule_lookup.insert((-1, 1), [-1, 1, 1, -1]);
    rule_lookup.insert((0, 1), [-1, 2, 1, -2]);
    rule_lookup.insert((1, 1), [-1, 1, 1, -1]);
    rule_lookup.insert((-2, 0), [-2, 0, 2, 0]);
    rule_lookup.insert((-1, 0), [-2, 1, 2, -1]);
    rule_lookup.insert((0, 0), [-2, 2, 2, -2]);
    rule_lookup.insert((1, 0), [-2, 1, 2, -1]);
    rule_lookup.insert((2, 0), [-2, 0, 2, 0]);
    rule_lookup.insert((-1, -1), [-1, 1, 1, -1]);
    rule_lookup.insert((0, -1), [-1, 2, 1, -2]);
    rule_lookup.insert((1, -1), [-1, 1, 1, -1]);
    rule_lookup.insert((0, -2), [0, 2, 0, -2]);

    walk_pad(&pad, &rule_lookup, (-2, 0), data);
}

fn main() {
    let data = include_str!("../input");

    println!("Round 1");
    round1(data);
    println!("Round 2");
    round2(data);
}
