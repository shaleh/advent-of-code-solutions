use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{char, digit1, one_of},
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a [u8]>;

fn find_adjacent_symbol(
    specific: Option<u8>,
    symbols: &HashMap<usize, u8>,
    row_length: usize,
    begin: usize,
    end: usize,
) -> Option<usize> {
    for pos in begin..end {
        for index in [
            pos.checked_sub(row_length),
            pos.checked_sub(row_length - 1),
            pos.checked_sub(row_length + 1),
            pos.checked_add(row_length),
            pos.checked_add(row_length - 1),
            pos.checked_add(row_length + 1),
        ]
        .into_iter()
        .flatten()
        {
            match symbols.get(&index) {
                sym @ Some(_) if sym == specific.as_ref() => return Some(index),
                Some(_) if specific.is_none() => return Some(index),
                _ => {}
            }
        }
    }

    if let Some(index) = begin.checked_sub(1) {
        match symbols.get(&index) {
            sym @ Some(_) if sym == specific.as_ref() => return Some(index),
            Some(_) if specific.is_none() => return Some(index),
            _ => {}
        }
    }
    match symbols.get(&end) {
        sym @ Some(_) if sym == specific.as_ref() => return Some(end),
        Some(_) if specific.is_none() => return Some(end),
        _ => {}
    }

    None
}

fn part1(input: &[NumberWithPosition], row_length: usize, symbols: &HashMap<usize, u8>) {
    let result: u32 = input
        .iter()
        .filter_map(|item| {
            find_adjacent_symbol(None, symbols, row_length, item.begin, item.end)
                .map(|_| item.number)
        })
        .sum();

    println!("{:?}", result);
}

fn part2(input: &[NumberWithPosition], row_length: usize, symbols: &HashMap<usize, u8>) {
    let mut unpaired: HashMap<usize, u32> = HashMap::new();
    let mut sum = 0;

    for item in input {
        if let Some(index) =
            find_adjacent_symbol(Some(b'*'), symbols, row_length, item.begin, item.end)
        {
            match unpaired.get(&index) {
                Some(value) => {
                    sum += value * item.number;
                }
                None => {
                    unpaired.insert(index, item.number);
                }
            }
        }
    }

    println!("{:?}", sum);
}

#[derive(Debug)]
struct NumberWithPosition {
    number: u32,
    begin: usize,
    end: usize,
}

#[derive(Debug)]
struct SymbolWithPosition {
    symbol: u8,
    position: usize,
}

#[derive(Debug)]
enum SymbolOrNumber {
    Symbol(SymbolWithPosition),
    Number(NumberWithPosition),
}

fn symbol(input: Span) -> IResult<Span, SymbolWithPosition> {
    let (input, begin) = position(input)?;
    let (input, sym) = one_of("!@#$%^&*()-+=/")(input)?;
    Ok((
        input,
        SymbolWithPosition {
            symbol: sym as u8,
            position: begin.location_offset(),
        },
    ))
}

fn number(input: Span) -> IResult<Span, NumberWithPosition> {
    let (input, begin) = position(input)?;
    let (input, num) = digit1(input)?;
    let (input, end) = position(input)?;
    Ok((
        input,
        NumberWithPosition {
            number: std::str::from_utf8(num.fragment()).unwrap().parse::<u32>().expect("a number"),
            begin: begin.location_offset(),
            end: end.location_offset(),
        },
    ))
}

fn symbol_or_number(input: Span) -> IResult<Span, SymbolOrNumber> {
    symbol(input)
        .map(|(input, sym)| (input, SymbolOrNumber::Symbol(sym)))
        .or_else(|_| number(input).map(|(input, num)| (input, SymbolOrNumber::Number(num))))
}

fn parse(input: Span) -> IResult<Span, Vec<SymbolOrNumber>> {
    many1(delimited(
        many0(alt((char('.'), char('\n')))),
        symbol_or_number,
        many0(alt((char('.'), char('\n')))),
    ))(input)
}

fn main() {
    let input = include_str!("../../day3/input").as_bytes();
    let row_length = input
        .iter()
        .position(|&c| c == b'\n')
        .expect("a newline is found")
        + 1;
    let (_, symbols_or_numbers) = parse(Span::new(input)).expect("it worked");
    let symbols: HashMap<usize, u8> = symbols_or_numbers
        .iter()
        .filter_map(|x| match x {
            SymbolOrNumber::Symbol(sym) => Some((sym.position, sym.symbol)),
            _ => None,
        })
        .collect();
    let numbers: Vec<NumberWithPosition> = symbols_or_numbers
        .into_iter()
        .filter_map(|x| match x {
            SymbolOrNumber::Number(num) => Some(num),
            _ => None,
        })
        .collect();

    part1(&numbers, row_length, &symbols);
    part2(&numbers, row_length, &symbols);
}
