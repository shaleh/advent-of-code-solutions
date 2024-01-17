use std::fmt::Debug;
use std::io::{self, BufRead};

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

    while let Ok(count) = handle.read_line(&mut buffer) {
        if count == 0 {
            // Should not happen.
            break;
        }
        let line = buffer.trim().to_string();
        if line.is_empty() {
            // Signifies the end of the stack definitions.
            break;
        }

        if line.starts_with('[') || line.starts_with(' ') {
            let mut columns = Vec::new();
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '[' {
                    let crate_name = chars.next().unwrap();
                    columns.push(crate_name);
                    chars.next().unwrap();
                } else if c == ' ' {
                    if let Some(space) = chars.peek() {
                        if *space == ' ' {
                            chars.next().unwrap();
                            chars.next().unwrap();
                            chars.next().unwrap();
                            columns.push(' ');
                        }
                    }
                }
            }
            initial_stacks.push(columns);
        } else {
            // column numbers row
            let columns: Vec<_> = line
                .split(' ')
                .filter_map(|x| (!x.is_empty()).then(|| x.parse::<usize>().unwrap()))
                .collect();
            for (index, stack) in initial_stacks.iter().enumerate() {
                // Validate the stack parsing.
                assert_eq!(stack.len(), columns.len(), "index {} does not match", index);
            }
            break;
        }

        buffer.clear();
    }

    // The rest of the input is moves.
    let moves = handle
        .lines()
        .filter_map(|line| {
            let data: Vec<_> = line
                .unwrap()
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            if data.is_empty() {
                None
            } else {
                Some(Move::new(&data))
            }
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
