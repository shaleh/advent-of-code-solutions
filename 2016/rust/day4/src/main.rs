use itertools::Itertools;
use std::cmp::{Ord, Ordering};

fn parse_input(input: &str) -> Option<(String, u32)> {
    let (room, sector_and_checksum) = input.rsplit_once('-').unwrap();
    let (sector, checksum) = sector_and_checksum.split_once('[').unwrap();
    let sector: u32 = sector.parse().expect("a valid number");
    let checksum = checksum.trim_end_matches(']');

    let letters: String = room
        .chars()
        .filter(|c| *c != '-')
        .sorted()
        .group_by(|x| *x)
        .into_iter()
        .map(|(key, group)| (group.count(), key))
        .sorted_by(|a, b| match Ord::cmp(&b.0, &a.0) {
            Ordering::Equal => Ord::cmp(&a.1, &b.1),
            order => order,
        })
        .map(|x| x.1)
        .take(5)
        .collect();

    if checksum == letters {
        Some((room.replace('-', " "), sector))
    } else {
        None
    }
}

fn decrypt(data: &str, key: u32) -> String {
    let a_value = 'a' as u32;

    data.chars()
        .map(|c| {
            if c == ' ' {
                c
            } else {
                let value = c as u32;
                // char into ascii value, subtract 'a' to get alphabet value.
                // add key to shift
                // mod to constrain to the alphabet. 0 to 25.
                // add 'a' to shift back into a ascii.
                char::from_u32((((value - a_value) + key) % 26) + a_value).unwrap()
            }
        })
        .collect()
}

fn main() {
    let data = include_str!("../input");
    let real_data: Vec<_> = data.lines().filter_map(parse_input).collect();
    println!(
        "{}",
        real_data.iter().map(|(_, sector)| sector).sum::<u32>()
    );
    for (encrypted, sector) in real_data {
        println!("{sector} {}", decrypt(&encrypted, sector));
    }
}

#[test]
fn test() {
    let tests = [
        (
            "aaaaa-bbb-z-y-x-123[abxyz]",
            Some(("aaaaa bbb z y x".to_string(), 123)),
        ),
        (
            "a-b-c-d-e-f-g-h-987[abcde]",
            Some(("a b c d e f g h".to_string(), 987)),
        ),
        (
            "not-a-real-room-404[oarel]",
            Some(("not a real room".to_string(), 404)),
        ),
        ("totally-real-room-200[decoy]", None),
        ("aaaaa-bbb-a-b-a-123[abxyz]", None),
    ];

    for (item, expected) in tests {
        assert_eq!(parse_input(item), expected);
    }
}
