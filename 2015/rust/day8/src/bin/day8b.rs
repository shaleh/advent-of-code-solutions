#![feature(exclusive_range_pattern)]

use advent_support::read_input;

fn raw_input_length(input: &str) -> usize {
    input.len()
}

fn input_length(input: &str) -> usize {
    let mut count = 0;
    let mut escaped = false;
    let mut hex_count = 0;

    for c in input.chars() {
        if escaped {
            match c {
                '"' | '\\' => {
                    escaped = false;
                    count += 1;
                }
                'x' if hex_count == 0 => {
                    hex_count = 2;
                }
                'a'..='f' | '0'..='9' if hex_count > 0 => {
                    hex_count -= 1;
                    if hex_count == 0 {
                        escaped = false;
                        count += 1;
                    }
                }
                _ => panic!("invalid escape {}", c),
            }
        } else {
            count += match c {
                '"' => 0,
                '\\' => {
                    escaped = true;
                    0
                }
                _ => 1,
            }
        }
    }
    count
}

fn main() {
    let mut raw_count = 0;
    let mut count = 0;

    let lines = read_input::<String>().expect("Invalid input");

    let mut idx = 0;
    
    for line in lines {
        idx += 1;
        println!("{}", idx);
        raw_count += raw_input_length(&line);
        let escaped_line = format!(r#""{}""#, line.escape_default());
        count += raw_input_length(&escaped_line);
    }

    println!("Raw {} Evaluated {} Delta {}", raw_count, count, count - raw_count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_raw_empty_string_length_2() {
        assert_eq!(2, raw_input_length(r#""""#));
    }

    #[test]
    fn test_raw_abc_is_5_chars() {
        assert_eq!(5, raw_input_length(r#""abc""#));
    }

    #[test]
    fn test_raw_a_string_is_10() {
        assert_eq!(10, raw_input_length(r#""aaa\"aaa""#));
    }

    #[test]
    fn test_raw_hex_apostrophe_is_six() {
        assert_eq!(6, raw_input_length(r#""\x27""#));
    }

    #[test]
    fn test_empty_string_length_6() {
        let escaped = format!(r#""{}""#, r#""""#.escape_default());
        assert_eq!(6, raw_input_length(&escaped));
    }

    #[test]
    fn test_abc_is_9_chars() {
        let escaped = format!(r#""{}""#, r#""abc""#.escape_default());
        assert_eq!(9, raw_input_length(&escaped));
    }

    #[test]
    fn test_a_string_is_16() {
        let escaped = format!(r#""{}""#, r#""aaa\"aaa""#.escape_default());
        assert_eq!(16, raw_input_length(&escaped));
    }

    #[test]
    fn test_hex_apostrophe_is_11() {
        let escaped = format!(r#""{}""#, r#""\x27""#.escape_default());
        assert_eq!(11, raw_input_length(&escaped));
    }

    #[test]
    fn test_example() {
        assert_eq!(22, raw_input_length(r#""\xa8br\x8bjr\"""#.escape_default().to_string().as_ref()));
    }
}
