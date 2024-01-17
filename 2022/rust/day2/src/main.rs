use advent_support::read_input;

#[derive(Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 1,
    Win = 2,
}

impl TryFrom<i32> for Outcome {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Lose),
            1 => Ok(Self::Draw),
            2 => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<i32> for Shape {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Rock),
            2 => Ok(Self::Paper),
            3 => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn outcome(p1: Shape, p2: Shape) -> i32 {
    match (p1 as i32 - p2 as i32) % 3 {
        0 => 1,
        2 | -1 => 2,
        _ => 0,
    }
}

fn part_one(input: &[String]) {
    let rounds = input.iter().map(|line| {
        let (p1, p2) = line.split_once(' ').unwrap();
        (
            Shape::try_from(p1.chars().next().unwrap() as i32 - 'A' as i32 + 1).unwrap(),
            Shape::try_from(p2.chars().next().unwrap() as i32 - 'X' as i32 + 1).unwrap(),
        )
    });
    let sum: i32 = rounds
        .map(|(p1, p2)| p2 as i32 + (3 * outcome(p1, p2)))
        .sum();
    println!("Score: {}", sum);
}

fn loses_against(shape: Shape) -> Shape {
    Shape::try_from(((shape as i32 + 1) % 3) + 1).unwrap()
}

fn draws_against(shape: Shape) -> Shape {
    shape
}

fn wins_against(shape: Shape) -> Shape {
    Shape::try_from((shape as i32 % 3) + 1).unwrap()
}

fn part_two(input: &[String]) {
    let rounds = input.iter().map(|line| {
        let (p1, p2) = line.split_once(' ').unwrap();
        (
            Shape::try_from(p1.chars().next().unwrap() as i32 - 'A' as i32 + 1).unwrap(),
            Outcome::try_from(p2.chars().next().unwrap() as i32 - 'X' as i32).unwrap(),
        )
    });
    let sum: i32 = rounds
        .map(|(shape, outcome)| {
            let action = match outcome {
                Outcome::Lose => loses_against,
                Outcome::Draw => draws_against,
                Outcome::Win => wins_against,
            };
            action(shape) as i32 + (outcome as i32 * 3)
        })
        .sum();
    println!("Score: {}", sum);
}

fn main() {
    let input = read_input().unwrap();
    part_one(&input);
    part_two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, Arbitrary, Gen};

    #[test]
    fn test_rock_loses_against_paper() {
        assert_eq!(outcome(Shape::Rock, Shape::Paper), 2);
    }

    #[test]
    fn test_scissors_wins_aganst_paper() {
        assert_eq!(outcome(Shape::Scissors, Shape::Paper), 0);
    }

    impl Arbitrary for Shape {
        fn arbitrary(g: &mut Gen) -> Self {
            let vals = &[Shape::Rock, Shape::Paper, Shape::Scissors];
            g.choose(vals).expect("choose value").clone()
        }
    }

    quickcheck! {
        fn prop_inverse(shape: Shape) -> bool {
            let winner = wins_against(shape);
            loses_against(winner) == shape
        }
    }
}
