use std::collections::BinaryHeap;

use advent_support::read_input;
use itertools::Itertools;

type Food = u32;
type Elf = Vec<Food>;

fn process_input(input: Vec<String>) -> Vec<Elf> {
    input
        .into_iter()
        .map(|x| x.parse::<Food>())
        .group_by(|x| x.is_ok())
        .into_iter()
        .filter_map(|(key, group)| key.then(|| group.collect::<Result<_, _>>().unwrap()))
        .collect()
}

fn part_one(total_calories_per_elf: &BinaryHeap<Food>) {
    println!("{}", total_calories_per_elf.iter().next().unwrap());
}

fn part_two(total_calories_per_elf: &BinaryHeap<Food>) {
    println!("{}", total_calories_per_elf.iter().take(3).sum::<Food>());
}

fn main() {
    let input = process_input(read_input().unwrap());
    let total_calories_per_elf =
        BinaryHeap::from(input.into_iter().map(|x| x.iter().sum()).collect::<Elf>());

    part_one(&total_calories_per_elf);
    part_two(&total_calories_per_elf);
}
