use std::collections::HashMap;
use std::fs;

fn part1(fs_size: &HashMap<String, u64>) {
    let total: u64 = fs_size
        .iter()
        .filter_map(|(_, &value)| (value <= 100_000).then_some(value))
        .sum();
    println!("total: {}", total);
}

fn part2(fs_size: &HashMap<String, u64>) {
    let total_storage_size = 70_000_000;
    let required_space = 30_000_000;
    let used_storage: u64 = *fs_size.get("/").expect("root");
    let remaining = total_storage_size - used_storage;
    println!("Remaining: {}", remaining);
    let missing_free_space = required_space - remaining;
    println!("Needs to delete {}", missing_free_space);
    let mut candidates: Vec<_> = fs_size
        .iter()
        .filter(|(key, value)| *value >= &missing_free_space)
        .collect();
    candidates.sort_unstable_by_key(|(_, value)| *value);
    println!("Remove: {:?}", candidates.first());
}

fn main() {
    let input_raw = fs::read_to_string("../inputs/7").expect("data");
    let mut lines: Vec<_> = input_raw.lines().rev().collect();
    let mut fs_size: HashMap<String, u64> = HashMap::new();
    let mut path: Vec<_> = Vec::new();
    while let Some(line) = lines.pop() {
        let parts: Vec<_> = line.split(' ').collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    path.pop();
                } else {
                    path.push(parts[2]);
                }
            }
        } else if parts[0] == "dir" {
            // ???
        } else {
            let size: u64 = parts[0].parse().expect("a number");
            for i in 0..path.len() {
                let entry = fs_size.entry(path[0..i + 1].join("/")).or_insert(0);
                *entry += size;
            }
        }
    }

    part1(&fs_size);
    part2(&fs_size);
}
