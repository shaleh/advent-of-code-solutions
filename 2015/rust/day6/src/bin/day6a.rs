extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fmt;

use advent_support::read_input;

use anyhow::Result;
use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "commandA.pest"]
pub struct LightCommand;

#[derive(Debug, Eq, PartialEq)]
enum Action {
    On,
    Off,
    Toggle,
}

impl From<&str> for Action {
    fn from(i: &str) -> Self {
        match i {
            "turn on" => Action::On,
            "turn off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => unimplemented!(""),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct NumericRange {
    begin: usize,
    end: usize,
}

impl NumericRange {
    fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Light {
    Off,
    On,
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Light::On => "O",
                Light::Off => "X",
            }
        )
    }
}

struct LightState {
    lights: Vec<Light>,
    rows: usize,
    columns: usize,
}

impl LightState {
    fn update(&mut self, action: Action, range1: NumericRange, range2: NumericRange) {
        for row in range1.end..=range2.end {
            for column in range1.begin..=range2.begin {
                // println!("{:?} {:?}", row, column);
                let index = (self.rows * row) + column;
                let light = &mut self.lights[index];
                *light = match action {
                    Action::Toggle => {
                        if *light == Light::Off {
                            Light::On
                        } else {
                            Light::Off
                        }
                    }
                    Action::On => Light::On,
                    Action::Off => Light::Off,
                };
            }
        }
    }

    fn count(&self, light: Light) -> usize {
        self.lights.iter().filter(|x| **x == light).count()
    }
}

impl Default for LightState {
    fn default() -> Self {
        Self {
            lights: vec![Light::Off; 1_000 * 1000],
            rows: 1_000,
            columns: 1_000,
        }
    }
}

impl fmt::Display for LightState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for column in 0..self.columns {
                write!(f, "{}", self.lights[row * column])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn parse_action(parser: Pair<Rule>) -> Action {
    parser.as_str().into()
}

fn parse_range(parser: Pair<Rule>) -> NumericRange {
    let mut inner = parser.into_inner();

    let begin_as_str = inner.next().expect("number exists").as_str();
    let begin = begin_as_str.parse().expect("valid number");
    let end_as_str = inner.next().expect("number exists").as_str();
    let end = end_as_str.parse().expect("valid number");

    NumericRange::new(begin, end)
}

fn parse(input: &str) -> Result<(Action, NumericRange, NumericRange)> {
    let result = LightCommand::parse(Rule::light_command, input)?
        .next()
        .unwrap();
    let mut parts = result.into_inner();

    let action: Action = parse_action(parts.next().unwrap());
    let begin_range = parse_range(parts.next().unwrap());
    let end_range = parse_range(parts.next().unwrap());

    Ok((action, begin_range, end_range))
}

fn main() -> Result<()> {
    let lines = read_input::<String>().expect("Invalid input");

    let mut state = LightState::default();

    for line in lines {
        // println!("{:?}", &line);
        let (action, range1, range2) = parse(&line)?;
        //println!("{:?} {:?} {:?}", action, range1, range2);
        state.update(action, range1, range2);
    }

    // println!("{}", state);
    println!("{}", state.count(Light::On));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_lights_is_1_million() {
        let mut state = LightState::default();
        state.update(
            Action::On,
            NumericRange::new(0, 0),
            NumericRange::new(999, 999),
        );
        assert_eq!(state.count(Light::On), 1_000_000);
    }

    #[test]
    fn test_first_row_on() {
        let mut state = LightState::default();
        state.update(
            Action::On,
            NumericRange::new(0, 0),
            NumericRange::new(999, 0),
        );
        assert_eq!(state.count(Light::On), 1_000);
    }

    #[test]
    fn test_middle_rectangle_is_off() {
        let mut state = LightState::default();
        state.update(
            Action::On,
            NumericRange::new(0, 0),
            NumericRange::new(999, 999),
        );
        state.update(
            Action::Off,
            NumericRange::new(499, 499),
            NumericRange::new(500, 500),
        );
        assert_eq!(state.count(Light::On), 1_000_000 - 4);
    }
}
