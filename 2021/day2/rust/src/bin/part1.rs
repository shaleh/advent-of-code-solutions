use std::io::{self, BufRead};

pub fn read_input() -> Vec<(String, i32)>
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    handle.lines().map(|line| {
        let line = line.unwrap();
        let (action, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<i32>().unwrap();
        (action.to_string(), distance)
    }).collect()
}

fn main() {
    let input = read_input();

    let mut horizontal = 0;
    let mut depth = 0;

    for (action, distance) in input {
        match action.as_str() {
            "forward" => { horizontal += distance; },
            "up" => { depth -= distance; },
            "down" => { depth += distance; },
            _ => { unreachable!(); },
        }
    }
    println!("{}, {}: {}", horizontal, depth, horizontal * depth);
}
