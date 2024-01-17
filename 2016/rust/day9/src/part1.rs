use std::iter;

use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{alpha1, char, digit1};
use nom::combinator::map_res;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug)]
enum Block {
    Text(String),
    Compression((usize, usize)),
}

fn alpha(input: &str) -> IResult<&str, Block> {
    alpha1(input).map(|(new_input, parsed)| (new_input, Block::Text(parsed.to_string())))
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn compression_details(input: &str) -> IResult<&str, Block> {
    separated_pair(number, char('x'), number)(input)
        .map(|(new_input, parsed)| (new_input, Block::Compression(parsed)))
}

fn compression(input: &str) -> IResult<&str, Block> {
    delimited(char('('), compression_details, char(')'))(input)
}

fn block(input: &str) -> IResult<&str, Block> {
    alt((compression, alpha))(input)
}

fn decompress(input: &str) -> Result<String, String> {
    let mut result: Vec<String> = Vec::new();
    let mut next_input = input;
    //let mut counter = 0;
    loop {
        match block(next_input).map_err(|msg| msg.to_string())? {
            (input, Block::Text(text)) => {
                result.push(text);
                next_input = input;
            }
            (input, Block::Compression((length, count))) => {
                result.push(
                    iter::repeat(input[..length].to_string())
                        .take(count)
                        .join(""),
                );
                next_input = &input[length..];
            }
        }
        if next_input.is_empty() {
            break;
        }
        //counter += 1;
        // if counter == 1 {
        //     dbg!(&result);
        //     dbg!(&next_input[..5]);
        //     break;
        // }
    }
    Ok(result.join(""))
}

fn main() {
    let data = include_str!("../input");
    let result = decompress(data.trim()).expect("success");
    println!("{}", result.len());
}
