use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

fn signum(value: i64) -> i64 {
    match value.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn follow(&self, other: &Position) -> Option<Position> {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;

        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            Some(Position {
                x: self.x + signum(delta_x),
                y: self.y + signum(delta_y),
            })
        } else {
            None
        }
    }
}

fn part1(input: &[&str]) -> usize {
    solve(input, 2)
}

fn part2(input: &[&str]) -> usize {
    solve(input, 10)
}

fn solve(input: &[&str], num_knots: usize) -> usize {
    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut knots = vec![Position::default(); num_knots];
    tail_positions.insert(knots[num_knots - 1]);

    for line in input {
        let (direction, count) = line.split_once(' ').expect("direction and count");
        let count = count.parse::<i64>().expect("number");

        for _ in 0..count {
            match direction {
                "U" => {
                    knots[0].y += 1;
                }
                "R" => {
                    knots[0].x += 1;
                }
                "D" => {
                    knots[0].y -= 1;
                }
                "L" => {
                    knots[0].x -= 1;
                }
                _ => {
                    unreachable!();
                }
            }

            for i in 1..knots.len() {
                if let Some(new_pos) = knots[i].follow(&knots[i - 1]) {
                    knots[i] = new_pos;
                }
            }
            tail_positions.insert(knots[num_knots - 1]);
        }
    }

    tail_positions.len()
}

fn main() {
    let input_raw = fs::read_to_string("../../inputs/9").expect("data");
    let input: Vec<&str> = input_raw.lines().collect();

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];

        assert_eq!(part1(&data), 13);
    }

    #[test]
    fn test_example2() {
        let data = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

        assert_eq!(part2(&data), 36);
    }
}
