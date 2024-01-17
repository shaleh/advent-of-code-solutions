use std::convert::TryFrom;
use std::ops::ControlFlow;

use advent_support::read_input;

enum Direction {
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Direction, ()> {
        match value {
            '(' => Ok(Self::Up),
            ')' => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

struct Floor(i64);

impl Floor {
    fn new() -> Self {
        Self(0)
    }

    fn up(&mut self) {
        self.0 += 1;
    }

    fn down(&mut self) {
        self.0 -= 1;
    }

    fn up_checked(&mut self) -> Option<i64> {
        self.0 += 1;
        Some(self.0)
    }

    fn down_checked(&mut self) -> Option<i64> {
        if self.0 > 0 {
            self.0 -= 1;
            Some(self.0)
        } else {
            None
        }
    }

    fn value(&self) -> i64 {
        self.0
    }
}

fn compute_index(input: &[Direction]) -> i64 {
    let result = input.iter().enumerate().try_fold(
        (0, Floor::new()),
        |(_, mut floor), (index, direction)| {
            let result = match direction {
                Direction::Up => floor.up_checked(),
                Direction::Down => floor.down_checked(),
            };
            match result {
                None => ControlFlow::Break(index as i64),
                Some(_) => ControlFlow::Continue((index, floor)),
            }
        },
    );
    match result {
        ControlFlow::Break(v) => v,
        ControlFlow::Continue((index, _)) => index as i64,
    }
}

fn part_one(input: &[Vec<Direction>]) {
    for line in input {
        let final_floor = line.iter().fold(Floor::new(), |mut floor, d| {
            match d {
                Direction::Up => floor.up(),
                Direction::Down => floor.down(),
            };
            floor
        });
        println!("Final floor: {}", final_floor.value());
    }
}

fn part_two(input: &[Vec<Direction>]) {
    for directions in input {
        let index = compute_index(directions);
        // 1-based index.
        println!("Index {}", index + 1);
    }
}

fn process_input(input: Vec<String>) -> Vec<Vec<Direction>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Direction::try_from(c).unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let input = process_input(read_input::<String>().expect("Invalid input"));
    part_one(&input);
    part_two(&input);
}
