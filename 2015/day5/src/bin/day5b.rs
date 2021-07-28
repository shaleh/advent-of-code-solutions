use advent_support::read_input;

use itertools::izip;
use itertools::Itertools;

fn has_valid_pairs(data: &str) -> bool {
    let mut chunks: Vec<Vec<(char, char)>> = Vec::new();
    for (_, group) in izip!(data.chars(), data[1..].chars())
        .group_by(|elt| *elt)
        .into_iter()
    {
        let chunk: Vec<(char, char)> = group.collect();
        let tmp = if chunk.len() == 2 {
            &chunk[1..]
        } else {
            &chunk
        };

        chunks.push(tmp.into_iter().map(|x| *x).collect());
    }

    let count = chunks
        .into_iter()
        .flatten()
        .sorted()
        .group_by(|elt| *elt)
        .into_iter()
        .filter(|(_, x)| x.count() > 1)
        .count();
    count > 0
}

// fn has_valid_pairs(data: &str) -> bool {
//     for i in 0..(data.len() - 1) {
//         if data[i+2..].contains(&data[i..i+2]) {
//             return true;
//         }
//     }

//     false
// }

fn has_one_repeat_with_letter_between(chars: &[char]) -> bool {
    let triples = izip!(chars, chars.iter().skip(1), chars.iter().skip(2));
    let in_between_count = triples.filter(|(a, _, c)| a == c).count();

    in_between_count > 0
}

fn is_nice(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();

    has_valid_pairs(input) && has_one_repeat_with_letter_between(&chars)
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");

    let mut count = 0;

    for line in lines {
        if is_nice(&line) {
            println!("nice    {}", line);
            count += 1;
        } else {
            println!("naughty {}", line);
        }
    }

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_pair_is_false() {
        let input = "aa";
        assert!(has_valid_pairs(input) == false);
    }

    #[test]
    fn test_2_pair_is_true() {
        let input = "xyaxy";
        assert!(has_valid_pairs(input));
    }

    #[test]
    fn test_2_identical_pair_is_true() {
        let input = "bbbb";
        assert!(has_valid_pairs(input));
    }

    #[test]
    fn test_triplet_is_false() {
        let input = "bbb";
        assert!(!has_valid_pairs(input));
    }

    #[test]
    fn test_has_repeat_with_letter_between() {
        let input = "bcaaaq";
        let chars: Vec<char> = input.chars().collect();
        assert!(has_one_repeat_with_letter_between(&chars));
    }

    #[test]
    fn test_examples_pass() {
        assert!(!is_nice("enamqzfzjunnnkpe"));
        let input = "ieodomkazujcvgmujy";
        assert!(has_valid_pairs(input));
        let chars: Vec<char> = input.chars().collect();
        assert!(has_one_repeat_with_letter_between(&chars));
        assert!(is_nice("ieodomkazujcvgmujy"));
        assert!(is_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_nice("xxyxx"));
        assert!(is_nice("uxurcxstgmygtbstg"));

        assert!(is_nice("uurcxstgmygtbstg") == false);
        assert!(is_nice("ieodomkazucvgmuy") == false);
    }
}
