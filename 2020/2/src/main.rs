use std::convert::TryInto;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Policy {
    pos1: u32,
    pos2: u32,
    letter: char,
}

impl Policy {
    fn new(raw_range: &str, letter: &str) -> Self {
        let range_values: Vec<u32> = raw_range.split("-").map(|x| x.parse().unwrap()).collect();
        let pos1 = *range_values.first().unwrap();
        let pos2 = *range_values.last().unwrap();
        dbg!(pos1, pos2, letter);
        Self { pos1, pos2, letter: letter.chars().next().unwrap() }
    }
}


fn char_matches(c: Option<char>, to_match: char) -> bool {
    c.map_or(false, |c| c == to_match)
}

#[derive(Clone, Debug)]
struct Password(String);

impl Password {
    fn new(source: String) -> Self {
        Self(source)
    }

    fn is_valid(&self, policy: &Policy) -> bool {
        let pos1: usize = policy.pos1.try_into().unwrap();
        let pos2: usize = policy.pos2.try_into().unwrap();
        let char1 = self.0.chars().nth(pos1 - 1);
        let char2 = self.0.chars().nth(pos2 - 1);
        let result = i32::from(char_matches(char1, policy.letter)) + i32::from(char_matches(char2, policy.letter));
        result == 1
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

fn main() {
    let match_count = read_input().iter().filter(|(policy, password)| password.is_valid(&policy)).count();
    println!("Matches policy: {}", match_count);
}
