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

type Crate = char;
type Stack = Vec<Crate>;

fn read_input() -> (Vec<Stack>, Vec<Move>) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut initial_stacks: Vec<Stack> = Vec::new();
    let mut buffer = String::new();

    while handle.read_line(&mut buffer).is_ok() {
        if buffer.starts_with(" 1") {
            break;
        }
        let chars: Stack = buffer.trim().chars().collect();
        buffer.clear();
        let row: Stack = (0..=(chars.len() / 4))
            .map(|i| chars[(i * 4) + 1])
            .collect();
        initial_stacks.push(row);
    }

    // empty row
    handle.read_line(&mut buffer).unwrap();

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

fn transpose(v: &[Stack]) -> Vec<Stack> {
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
                .collect::<Stack>()
        })
        .collect()
}

fn part_one(initial_stack: &[Stack], moves: &[Move]) {
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

fn part_two(initial_stack: &[Stack], moves: &[Move]) {
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
