use std::collections::HashSet;

use itertools::Itertools;
use nom::character::complete::{alpha1, char};
use nom::sequence::delimited;
use nom::IResult;

fn supernet(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn hypernet(input: &str) -> IResult<&str, &str> {
    delimited(char('['), supernet, char(']'))(input)
}

fn parse_ip(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let mut supernets = Vec::new();
    let mut hypernets = Vec::new();

    let mut input = input.clone();
    loop {
        let (next_input, next_net) = supernet(input)?;
        supernets.push(next_net);
        if next_input.is_empty() {
            break;
        }
        input = next_input;
        let (next_input, next_hypernet) = hypernet(input)?;
        hypernets.push(next_hypernet);
        input = next_input;
    }

    Ok((input, (supernets, hypernets)))
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    match parse_ip(input) {
        Ok((_, result)) => result,
        Err(msg) => panic!("{}", msg),
    }
}

fn does_support_ssl(supernets: &[&str], hypernets: &[&str]) -> bool {
    let abas: HashSet<(_, _, _)> = supernets
        .iter()
        .flat_map(|x| {
            (*x).chars()
                .tuple_windows::<(_, _, _)>()
                // ABA
                .filter(|x| x.0 == x.2 && x.0 != x.1)
        })
        .collect();
    // Find the BAB, then invert it to be an ABA.
    let babs: HashSet<(_, _, _)> = hypernets
        .iter()
        .flat_map(|x| {
            (*x).chars()
                .tuple_windows::<(_, _, _)>()
                .filter(|x| x.0 == x.2 && x.0 != x.1)
        })
        .map(|x| (x.1, x.0, x.1))
        .collect();
    abas.intersection(&babs).next().is_some()
}

fn does_support_tls(supernets: &[&str], hypernets: &[&str]) -> bool {
    !hypernets.iter().any(|x| has_abba(x)) && supernets.iter().any(|x| has_abba(x))
}

fn has_abba(input: &str) -> bool {
    input.chars().tuple_windows::<(_, _, _, _)>().any(|window| {
        let (a, b, c, d) = window;
        a == d && b == c && a != b
    })
}

fn main() {
    let data = include_str!("../input");

    let parsed: Vec<(Vec<_>, Vec<_>)> = data.lines().map(parse).collect();

    println!(
        "Round 1: {}",
        parsed
            .iter()
            .filter(|(supernets, hypernets)| does_support_tls(supernets, hypernets))
            .count()
    );
    println!(
        "Round 2: {}",
        parsed
            .iter()
            .filter(|(supernets, hypernets)| does_support_ssl(supernets, hypernets))
            .count()
    );
}

#[test]
fn test_tls() {
    let cases = [
        ("abba[mnop]qrst", true),
        ("abcd[bddb]xyyx", false),
        ("aaaa[qwer]tyui", false),
        ("ioxxoj[asdfgh]zxcvbn", true),
    ];

    for (ip, expected) in cases {
        assert_eq!(does_support_tls(ip), expected, "{ip:?}");
    }
}

#[test]
fn test_ssl() {
    let cases = [
        ("abba[mnop]qrst", true),
        ("abcd[bddb]xyyx", false),
        ("aaaa[qwer]tyui", false),
        ("ioxxoj[asdfgh]zxcvbn", true),
    ];

    for (ip, expected) in cases {
        assert_eq!(does_support_tls(ip), expected, "{ip:?}");
    }
}
