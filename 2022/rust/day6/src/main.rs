use std::time::Instant;

use advent_support::read_input;
use std::collections::HashSet;

fn find_unique_window(size: usize, input: &str) -> Option<(usize, String)> {
    let mut begin = 0;
    let mut end = size;
    let chars: Vec<_> = input.chars().collect();
    while end <= input.len() {
        let mut seen = 0;
        let mut i = end;
        while i > begin {
            i -= 1;
            let ord = chars[i] as i32 - 'a' as i32;
            if seen & 1 << ord != 0 {
                end = end + (size - (end - i)) + 1;
                begin = i + 1;
                assert_eq!(end - begin, size);
                break;
            }
            seen |= 1 << ord;
        }
        if i == begin {
            return Some((end, input[begin..end].to_string()));
        }
    }

    None
}

fn fast_hash_find_unique_window(size: usize, input: &str) -> Option<(usize, String)> {
    let mut begin = 0;
    let mut end = size;
    let chars: Vec<_> = input.chars().collect();
    while end <= input.len() {
        let mut i = end;
        let mut seen: HashSet<char> = HashSet::with_capacity(size);
        while i > begin {
            i -= 1;

            if !seen.insert(chars[i]) {
                end = end + (size - (end - i)) + 1;
                begin = i + 1;
                assert_eq!(end - begin, size);
                break;
            }
        }
        if i == begin {
            return Some((end, input[begin..end].to_string()));
        }
    }

    None
}

fn hash_find_unique_window(size: usize, input: &str) -> Option<(usize, String)> {
    for i in 0..=input.len() {
        let window = &input[i..i+size];
        let unique_chars: HashSet<char> = window.chars().collect();
        if unique_chars.len() == size {
            return Some((i + size, window.into()));
        }
    }
    None
}

fn part_one(input: &str) {
    match find_unique_window(4, input) {
        Some((pos, window)) => {
            println!("part one: {} {}", window, pos);
        }
        None => {
            println!("No solution found");
        }
    }
}

fn part_two(input: &str) {
    let hash_result = fast_hash_find_unique_window(14, input);
    let result = find_unique_window(14, input);
    assert_eq!(hash_result, result);

    let start = Instant::now();
    for _ in 0..1_000 {
        fast_hash_find_unique_window(14, input);
    }
    let hashed_duration = start.elapsed();

    let start = Instant::now();
    for _ in 0..1_000 {
        find_unique_window(14, input);
    }
    let duration = start.elapsed();
    
    println!("{:?} v {:?}", duration, hashed_duration);

    match find_unique_window(14, input) {
        Some((pos, window)) => {
            println!("part one: {} {}", window, pos);
        }
        None => {
            println!("No solution found");
        }
    }
}

fn main() {
    let input = read_input().unwrap();

    part_one(&input[0]);
    part_two(&input[0]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_find_at_beginning() {
        assert_eq!(hash_find_unique_window(4, "abcde"), Some((4, "abcd".into())));
    }

    #[test]
    fn can_find_at_end() {
        assert_eq!(hash_find_unique_window(4, "bbcde"), Some((5, "bcde".into())));
    }
}
