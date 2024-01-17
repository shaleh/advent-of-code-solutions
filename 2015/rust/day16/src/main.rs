use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, multispace0};
use nom::combinator::map_res;
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;
use std::collections::HashMap;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn alpha(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn colon(input: &str) -> IResult<&str, char> {
    char(':')(input)
}

#[derive(Debug)]
struct AuntData<'a>(HashMap<&'a str, u64>);

impl<'a> AuntData<'a> {
    fn parse(input: &'a str) -> Self {
        let (input, _) = pair(ws(alpha), ws(number))(input).unwrap();
        let (input, _) = colon(input).unwrap();
        let (_, values) =
            separated_list1(tag(","), separated_pair(ws(alpha), char(':'), ws(number)))(input)
                .unwrap();

        let data: HashMap<&'a str, u64> = values.into_iter().collect();

        Self(data)
    }
}

fn check_for_match(needle: &HashMap<&str, u64>, haystack: &HashMap<&str, u64>) -> bool {
    needle
        .iter()
        .filter(|(&key, &value)| {
            haystack
                .get(&key)
                .map_or(false, |&potential_match| value == potential_match)
        })
        .count()
        == haystack.len()
}

fn check_for_ranged_match(needle: &HashMap<&str, u64>, haystack: &HashMap<&str, u64>) -> bool {
    needle
        .iter()
        .filter(|(&key, &value)| {
            haystack
                .get(&key)
                .map_or(false, |&potential_match| match key {
                    "cats" | "trees" => potential_match > value,
                    "goldfish" | "pomeranians" => potential_match < value,
                    _ => potential_match == value,
                })
        })
        .count()
        == haystack.len()
}

fn main() {
    let data = include_str!("../input");
    let data: Vec<AuntData> = data.lines().map(AuntData::parse).collect();
    println!("Aunt data {:?}", data);
    let ticker_values: HashMap<&str, u64> = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect();
    println!("{:?}", ticker_values);

    let candidates: Vec<_> = data
        .iter()
        .enumerate()
        .filter(|(_, aunt)| check_for_match(&ticker_values, &aunt.0))
        .collect();
    println!("Candidates: {:?}", candidates);

    let candidates: Vec<_> = data
        .iter()
        .enumerate()
        .filter(|(_, aunt)| check_for_ranged_match(&ticker_values, &aunt.0))
        .collect();
    println!("Candidates with ranges applied: {:?}", candidates);
}
