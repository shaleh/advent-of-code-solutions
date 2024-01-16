use itertools::multiunzip;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq)]
struct RGB {
    red: u32,
    green: u32,
    blue: u32,
}

impl RGB {
    fn add(&self, other: &RGB) -> RGB {
        RGB {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }

    fn tuple(&self) -> (u32, u32, u32) {
        (self.red, self.green, self.blue)
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn color(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(input)
}

fn single(input: &str) -> IResult<&str, RGB> {
    let (input, (number, name)) = separated_pair(u32, space1, color)(input)?;
    let rgb = match name {
        "red" => RGB {
            red: number,
            green: 0,
            blue: 0,
        },
        "green" => RGB {
            red: 0,
            green: number,
            blue: 0,
        },
        "blue" => RGB {
            red: 0,
            green: 0,
            blue: number,
        },
        _ => unimplemented!(),
    };
    Ok((input, rgb))
}

fn double(input: &str) -> IResult<&str, RGB> {
    let (input, (color1, color2)) =
        separated_pair(single, terminated(tag(","), space0), single)(input)?;
    Ok((input, color1.add(&color2)))
}

fn triple(input: &str) -> IResult<&str, RGB> {
    let (input, (color1, color2)) =
        separated_pair(double, terminated(tag(","), space0), single)(input)?;
    Ok((input, color1.add(&color2)))
}

fn parse_rgb_values(input: &str) -> IResult<&str, RGB> {
    alt((triple, double, single))(input)
}

// Game 1: 4 blue, 4 red, 16 green; 14 green, 5 red; 1 blue, 3 red, 5 green
fn game(input: &str) -> IResult<&str, (u32, Vec<RGB>)> {
    let (input, ((_, number), values)) = separated_pair(
        separated_pair(tag("Game"), space1, u32),
        terminated(tag(":"), space0),
        separated_list1(terminated(tag(";"), space0), parse_rgb_values),
    )(input)?;
    Ok((input, (number, values)))
}

fn valid_game(bag: &RGB, game: &RGB) -> bool {
    game.red <= bag.red && game.green <= bag.green && game.blue <= bag.blue
}

fn minimal_bag(games: &[RGB]) -> RGB {
    let (reds, greens, blues): (Vec<u32>, Vec<u32>, Vec<u32>) =
        multiunzip(games.iter().map(|g| g.tuple()));
    RGB {
        red: *reds.iter().max().unwrap(),
        green: *greens.iter().max().unwrap(),
        blue: *blues.iter().max().unwrap(),
    }
}

fn main() -> Result<(), String> {
    let input = include_str!("../input");
    let bag = RGB {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut possible = 0;
    let mut powers = 0;
    for game in input.lines().map(game) {
        match game {
            Ok((_, g)) => {
                let valid = g.1.iter().all(|x| valid_game(&bag, x));
                let minimal = minimal_bag(&g.1);
                if valid {
                    possible += g.0;
                }
                powers += minimal.power();
            }
            Err(e) => panic!("{:?}", e),
        }
    }
    println!("{possible}");
    println!("{powers}");
    Ok(())
}
