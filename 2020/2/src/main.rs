use std::convert::TryInto;
use core::ops::RangeInclusive;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Policy {
    range: RangeInclusive<u32>,
    letter: char,
}

impl Policy {
    fn new(raw_range: &str, letter: &str) -> Self {
        let range_values: Vec<u32> = raw_range.split("-").map(|x| x.parse().unwrap()).collect();
        let begin = range_values.first().unwrap();
        let end = range_values.last().unwrap();
        Self { range: (*begin..=*end), letter: letter.chars().next().unwrap() }
    }
}

#[derive(Clone, Debug)]
struct Password(String);

impl Password {
    fn new(source: String) -> Self {
        Self(source)
    }
}

fn read_input() -> Vec<(Policy, Password)> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input: Vec<(Policy, Password)> = Vec::new();
    for line in handle.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" ").collect();
        let counts = parts[0];
        let letter = parts[1].split_terminator(":").take(1).next().unwrap();
        let policy = Policy::new(counts, letter);
        let password = Password::new(String::from(parts[2]));
        input.push((policy, password))
    }
    input
}

fn honors_policy(password: &Password, policy: &Policy) -> bool {
    let char_count: u32 = password.0.chars().filter(|c| *c == policy.letter).count().try_into().unwrap();
    let result = policy.range.contains(&char_count);
    if result {
        result
    } else {
        dbg!(char_count, password, policy);
        result
    }
}

fn main() {
    let match_count = read_input().iter().filter(|(policy, password)| honors_policy(&password, &policy)).count();
    println!("Matches policy: {}", match_count);
}
