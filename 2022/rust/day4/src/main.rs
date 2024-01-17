use std::ops::RangeInclusive;

use advent_support::read_input;

fn has_full_overlap(one: &RangeInclusive<u32>, two: &RangeInclusive<u32>) -> bool {
    (one.start() >= two.start() && one.start() <= two.end() && one.end() <= two.end())
        || (two.start() >= one.start() && two.start() <= one.end() && two.end() <= one.end())
}

fn part_one(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(one, two)| has_full_overlap(one, two))
        .count()
}

fn has_any_overlap(one: &RangeInclusive<u32>, two: &RangeInclusive<u32>) -> bool {
    (one.start() >= two.start() && one.start() <= two.end())
        || (one.end() >= two.start() && one.end() <= two.end())
        || (two.start() >= one.start() && two.start() <= one.end())
        || (two.end() >= one.start() && two.end() <= one.end())
}

fn part_two(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(one, two)| has_any_overlap(one, two))
        .count()
}

fn parse(input: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let numbers: Vec<u32> = input
        .split(',')
        .flat_map(|x| x.split('-').map(|x| x.parse::<u32>().unwrap()))
        .collect();
    let range_one = numbers[0]..=numbers[1];
    let range_two = numbers[2]..=numbers[3];
    (range_one, range_two)
}

fn main() {
    let input = read_input().unwrap();
    let input: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = input
        .iter()
        .map(|line| parse(line))
        .collect();

    println!("Part one: {}", part_one(&input));
    println!("Part one: {}", part_two(&input));
}
