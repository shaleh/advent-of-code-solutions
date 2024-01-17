use md5::{Digest, Md5};

use advent_support::read_input;

fn brute_hash(input: &[String], num_zeroes: usize) -> i64 {
    let base = input[0].clone();
    let zeroes = "0".repeat(num_zeroes);

    let mut current: i64 = 1;
    loop {
        let mut hasher = Md5::new();

        let value = format!("{}{}", base, current);
        hasher.update(value);

        let digest = hasher.finalize();
        let hex_digest = format!("{:x}", digest);
        if &hex_digest[0..num_zeroes] == zeroes {
            return current;
        }

        current += 1;
    }
}

fn thing(goal: &str, value: i64) -> i64 {
    let mut hasher = Md5::new();

    let value = format!("{}{}", base, current);
    hasher.update(value);

    let digest = hasher.finalize();
    let hex_digest = format!("{:x}", digest);
    if &hex_digest[0..num_zeroes] == goal {
        Some(current)
    } else {
        None
    }
}

fn try_brute_hash(input: &[String], num_zeroes: usize) -> i64 {
    let base = input[0].clone();
    let zeroes = "0".repeat(num_zeroes);

    [1u64..].try_for_each(|current| {
    }
}

fn part_one(input: &[String]) {
    println!("Value: {}", brute_hash(&lines, 5));
}

fn part_two(input: &[String]) {
    println!("Value: {}", brute_hash(&lines, 6));
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    part_one(&lines);
    part_two(&lines);
}
