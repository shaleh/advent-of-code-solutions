use md5::{Digest, Md5};
use std::io::{stdout, Write};

fn find_password(input: &str, zeroes: usize, length: usize) -> String {
    let initial_zeroes = String::from_utf8(vec![b'0'; zeroes]).unwrap();

    let mut iterations: u64 = 0;
    let mut found: usize = 0;
    let mut result: Vec<u8> = vec![0; length];

    let mut stdout = stdout();
    print!("{:?}", result);
    stdout.flush().unwrap();

    while found < length {
        let mut md5 = Md5::new();
        md5.update(input);
        md5.update(&iterations.to_string());
        let digest = md5.finalize();
        let hex_digest = format!("{:x}", digest);
        let beginning = &hex_digest[..zeroes];
        if beginning == initial_zeroes {
            let hex_bytes = hex_digest.as_bytes();
            result[found] = hex_bytes[5];
            found += 1;
            print!("\r{:?}", result);
            stdout.flush().unwrap();
        }
        iterations += 1;
    }
    println!();

    String::from_utf8(result).expect("valid string")
}

fn from_hex(value: char) -> usize {
    match value as u32 {
        c @ (0x30..=0x39) => (c - 0x30/* zero */) as usize,
        c @ (0x61..=0x76) => (c - 0x61/* a */ + 10) as usize,
        _ => unreachable!(),
    }
}

fn find_password_round2(input: &str, zeroes: usize, length: usize) -> String {
    let initial_zeroes = String::from_utf8(vec![b'0'; zeroes]).unwrap();

    let mut iterations: u64 = 0;
    let mut found: usize = 0;
    let mut result: Vec<char> = vec!['_'; length];

    let mut stdout = stdout();

    print!("{:?}", result);
    stdout.flush().unwrap();

    while found < length {
        let mut md5 = Md5::new();
        md5.update(input);
        md5.update(&iterations.to_string());
        let digest = md5.finalize();
        let hex_digest = format!("{:x}", digest);
        let beginning = &hex_digest[..zeroes];
        if beginning == initial_zeroes {
            let mut hex_bytes = hex_digest.chars().skip(5);
            let position = from_hex(hex_bytes.next().unwrap());
            let new_char = hex_bytes.next().unwrap();

            if position < length && result[position] == '_' {
                found += 1;
                result[position] = new_char;
                print!("\r{:?}", result);
                stdout.flush().unwrap();
            }
        }
        iterations += 1;
    }
    println!();
    String::from_iter(result)
}

fn main() {
    let input = include_str!("../input");
    let password = find_password(input.trim(), 5, 8);
    println!("Round 1: {password}");
    let password = find_password_round2(input.trim(), 5, 8);
    println!("Round 2: {password}");
}
