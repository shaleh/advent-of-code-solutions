#![feature(iterator_fold_self)]

use std::io::{self, BufRead};

fn read_input() -> Vec<Vec<String>> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut acc: Vec<Vec<String>> = Vec::new();
    let mut in_progress: Vec<String> = Vec::new();
    for line in handle.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            acc.push(in_progress.to_vec());
            in_progress.clear();
        } else {
            in_progress.push(line);
        }
    }

    if !in_progress.is_empty() {
        acc.push(in_progress.into_iter().collect());
    }

    acc
}

fn make_set(data: &[String]) -> u32 {
    data.iter()
        .map(|x| compute_one_set(&x))
        .fold_first(|acc, value| acc & value)
        .unwrap_or(0)
}

fn compute_one_set(data: &str) -> u32 {
    let mut set = 0b0000_0000u32;

    for c in data.bytes() {
        set |= 0b0000_0001 << (c - b'a');
    }

    set
}

fn count_bits(value: u32) -> u32 {
    if value == 0 {
        return 0;
    }

    let mut count = 0;
    let mut mask: u32 = 1;
    while mask < 2u32.pow(27) {
        if (value & mask) != 0 {
            count += 1;
        }
        mask <<= 1;
    }
    count
}

fn main() {
    let data = read_input();
    let sets: Vec<u32> = data.into_iter().map(|x| make_set(&x)).collect();
    let total = sets.into_iter().map(count_bits).sum::<u32>();
    println!("Total: {}", total);
}
