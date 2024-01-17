use std::fmt::Debug;
use std::io::{self, BufRead};

use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Move {
    count: usize,
    source: usize,
    destination: usize,
}

impl Move {
    fn new(data: &[usize]) -> Self {
        Move {
            count: data[0],
            source: data[1] - 1,
            destination: data[2] - 1,
        }
    }
}

fn read_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut initial_stacks: Vec<Vec<char>> = Vec::new();
    let mut buffer = String::new();

    let stack_regex = Regex::new(r"(?:[A-Z]|   ) ?").unwrap();
    let column_indexes_regex = Regex::new(r"\d+").unwrap();
    let moves_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    while handle.read_line(&mut buffer).is_ok() {
        if buffer.starts_with(" 1") {
            // column numbers row
            let indexes: Vec<_> = column_indexes_regex
                .find_iter(&buffer)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect();
            // Validate the stack parsing.
            for (index, stack) in initial_stacks.iter().enumerate() {
                assert_eq!(stack.len(), indexes.len(), "index {} does not match", index);
            }
            break;
        } else {
            let columns = stack_regex
                .find_iter(&buffer)
                .map(|m| match m.as_str() {
                    "   " => ' ',
                    value => value.chars().take(1).next().unwrap(),
                })
                .collect();

            initial_stacks.push(columns);
            buffer.clear();
        }
    }

    // The rest of the input is moves.
    let moves = handle
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            moves_regex.captures(&line).map(|captures| {
                let data: Vec<_> = captures
                    .iter()
                    .filter_map(|x| x.unwrap().as_str().parse::<usize>().ok())
                    .collect();
                Move::new(&data)
            })
        })
        .collect();

    (initial_stacks, moves)
}

fn transpose(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.iter().map(|n| n.iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|n| match n.next().unwrap() {
                    // Now to drop empty elements in a stack.
                    ' ' => None,
                    value => Some(value),
                })
                .copied()
                .rev() // reversed to allow popping later.
                .collect::<Vec<char>>()
        })
        .collect()
}

fn part_one(initial_stack: &[Vec<char>], moves: &[Move]) {
    let mut stacks = transpose(initial_stack);
    for step in moves {
        for _ in 0..step.count {
            if let Some(value) = stacks[step.source].pop() {
                stacks[step.destination].push(value);
            }
        }
    }

    let top_crates: String = stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();
    println!("{}", top_crates);
}

fn part_two(initial_stack: &[Vec<char>], moves: &[Move]) {
    let mut stacks = transpose(initial_stack);

    for step in moves {
        let mut tmp: Vec<_> = Vec::new();

        for _ in 0..step.count {
            if let Some(value) = stacks[step.source].pop() {
                tmp.push(value);
            }
        }
        for _ in 0..tmp.len() {
            let value = tmp.pop().unwrap();
            stacks[step.destination].push(value);
        }
    }

    let top_crates: String = stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();
    println!("{}", top_crates);
}

fn main() {
    let (initial_stack, moves) = read_input();
    part_one(&initial_stack, &moves);
    part_two(&initial_stack, &moves);
}
