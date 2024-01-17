use std::collections::HashSet;

use advent_support::read_input;

fn into_priority(item: char) -> u32 {
    if ('A'..='Z').contains(&item) {
        item as u32 - 'A' as u32 + 27
    } else {
        item as u32 - 'a' as u32 + 1
    }
}

fn find_item(items: &Vec<char>) -> u32 {
    let size = items.len();
    let left: HashSet<char> = items[0..size / 2].iter().copied().collect();
    let right: HashSet<char> = items[size / 2..].iter().copied().collect();
    let mut intersection = left.intersection(&right);
    into_priority(*(intersection.next().unwrap()))
}

fn part_one(rucksacks: &[Vec<char>]) -> u32 {
    rucksacks.iter().map(find_item).sum()
}

fn find_badge(triplet: &[Vec<char>]) -> u32 {
    let one: HashSet<char> = HashSet::from_iter(triplet[0].iter().copied());
    let two = HashSet::from_iter(triplet[1].iter().copied());
    let three = HashSet::from_iter(triplet[2].iter().copied());

    let all_items: HashSet<char> = one.intersection(&two).copied().collect();
    let mut all_items = all_items.intersection(&three).copied();
    let badge = all_items.next().unwrap();
    into_priority(badge)
}

fn part_two(rucksacks: &[Vec<char>]) -> u32 {
    rucksacks.chunks(3).map(find_badge).sum()
}

fn main() {
    let input: Vec<String> = read_input();
    let input = input.into_iter().map(|line| line.chars().collect()).collect();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
