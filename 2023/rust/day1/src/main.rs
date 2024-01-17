use std::cmp::max;

struct NumberIterator<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> NumberIterator<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }
}

const NUMBERS: [(&str, u32); 18] = [
    ("eight", 8),
    ("five", 5),
    ("four", 4),
    ("nine", 9),
    ("one", 1),
    ("seven", 7),
    ("six", 6),
    ("three", 3),
    ("two", 2),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

impl<'a> Iterator for NumberIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.s.len() {
            for (word, number) in NUMBERS {
                if let Some(v) = self.s.get(self.pos..self.pos + word.len()) {
                    if v == word {
                        self.pos += max(1, word.len() - 1);
                        return Some(number);
                    }
                }
            }
            self.pos += 1;
        }

        None
    }
}

fn first_and_last_1(input: &str) -> u32 {
    let mut it = input.chars().filter(|c| c.is_ascii_digit());
    let first = it.next().unwrap() as u32;
    let last = it.last().map(|c| c as u32).unwrap_or(f);

    // 48 is ascii zero.
    ((first - 48) * 10) + (last - 48)
}

fn first_and_last_2(input: &str) -> u32 {
    let mut it = NumberIterator::new(input);
    let first = it.next().unwrap();
    let last = it.last().unwrap_or(f);

    (first * 10) + last
}

fn main() {
    let input = include_str!("../input");

    let sum1: u32 = input.lines().map(first_and_last_1).sum();
    println!("{sum1}");

    let sum2: u32 = input.lines().map(first_and_last_2).sum();
    println!("{sum2}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn inputs() {
        assert_eq!(first_and_last_2("two1nine"), 29);
        assert_eq!(first_and_last_2("eightwothree"), 83);
        assert_eq!(first_and_last_2("abcone2threexyz"), 13);
        assert_eq!(first_and_last_2("xtwone3four"), 24);
        assert_eq!(first_and_last_2("4nineeightseven2"), 42);
        assert_eq!(first_and_last_2("zoneight234"), 14);
        assert_eq!(first_and_last_2("7pqrstsixteen"), 76);
        assert_eq!(first_and_last_2("twone"), 21);
    }
}
