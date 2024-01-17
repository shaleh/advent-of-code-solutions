use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn repetition_code(
    input: &[Vec<char>],
    sorter: fn(a: &(usize, &char), b: &(usize, &char)) -> Ordering,
) -> String {
    input
        .iter()
        .map(|row| {
            row.iter()
                .sorted()
                .group_by(|x| *x)
                .into_iter()
                .map(|(key, group)| (group.count(), key))
                .sorted_by(sorter)
                .map(|x| x.1)
                .next()
                .unwrap()
        })
        .collect()
}

fn most_common(a: &(usize, &char), b: &(usize, &char)) -> Ordering {
    match Ord::cmp(&b.0, &a.0) {
        Ordering::Equal => Ord::cmp(&a.1, &b.1),
        order => order,
    }
}

fn least_common(a: &(usize, &char), b: &(usize, &char)) -> Ordering {
    match Ord::cmp(&a.0, &b.0) {
        Ordering::Equal => Ord::cmp(&a.1, &b.1),
        order => order,
    }
}

fn transposed_method() {
    let data = include_str!("../input");
    let input: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let column_count = input[0].len();
    let row_count = input.len();

    let transposed: Vec<Vec<char>> = (0..column_count)
        .map(|col| {
            (0..row_count)
                .map(|row| input[row][col])
                .collect::<Vec<char>>()
        })
        .collect();

    let result = repetition_code(&transposed, most_common);
    println!("Round1: {result}");

    let result = repetition_code(&transposed, least_common);
    println!("Round2: {result}");
}

fn counter_method() {
    let data = include_str!("../input");
    let mut counters = Vec::new();
    for line in data.lines() {
        let delta = line.len() - counters.len();
        for _ in 0..delta {
            counters.push(HashMap::new());
        }
        for (idx, c) in line.chars().enumerate() {
            counters[idx]
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    let round1: String = counters
        .iter()
        .map(|counter| {
            counter
                .iter()
                .sorted_by(|a, b| most_common(&(*a.1, a.0), &(*b.1, b.0)))
                .map(|x| *x.0)
                .next()
                .unwrap()
        })
        .collect();
    println!("Round1: {round1}");
    let round2: String = counters
        .iter()
        .map(|counter| {
            counter
                .iter()
                .sorted_by(|a, b| least_common(&(*a.1, a.0), &(*b.1, b.0)))
                .map(|x| *x.0)
                .next()
                .unwrap()
        })
        .collect();
    println!("Round2: {round2}");
}

fn main() {
    transposed_method();
    counter_method();
}
