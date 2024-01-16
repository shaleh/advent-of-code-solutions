use std::collections::HashMap;

fn find_adjacent_symbol(
    specific: Option<u8>,
    symbols: &HashMap<usize, u8>,
    row_length: usize,
    begin: usize,
    end: usize,
) -> Option<usize> {
    for pos in begin..end {
        for index in [
            pos.checked_sub(row_length),
            pos.checked_sub(row_length - 1),
            pos.checked_sub(row_length + 1),
            pos.checked_add(row_length),
            pos.checked_add(row_length - 1),
            pos.checked_add(row_length + 1),
        ]
        .into_iter()
        .flatten()
        {
            match symbols.get(&index) {
                sym @ Some(_) if sym == specific.as_ref() => return Some(index),
                Some(_) if specific.is_none() => return Some(index),
                _ => {}
            }
        }
    }

    if let Some(index) = begin.checked_sub(1) {
        match symbols.get(&index) {
            sym @ Some(_) if sym == specific.as_ref() => return Some(index),
            Some(_) if specific.is_none() => return Some(index),
            _ => {}
        }
    }
    match symbols.get(&end) {
        sym @ Some(_) if sym == specific.as_ref() => return Some(end),
        Some(_) if specific.is_none() => return Some(end),
        _ => {}
    }

    None
}

fn part1(input: &[u8], row_length: usize, symbols: &HashMap<usize, u8>) {
    let mut sum = 0;

    let mut cursor = 0;
    while cursor < input.len() {
        let mut current = cursor;
        unsafe {
            while input.get_unchecked(current).is_ascii_digit() {
                current += 1;
            }
        }
        if current == cursor {
            current += 1;
        } else if let Some(_) = find_adjacent_symbol(None, symbols, row_length, cursor, current) {
            let num: u32 = std::str::from_utf8(&input[cursor..current])
                .expect("a string")
                .parse()
                .expect("a number");

            sum += num;
        }
        cursor = current;
    }

    println!("{:?}", sum);
}

fn part2(input: &[u8], row_length: usize, symbols: &HashMap<usize, u8>) {
    let mut unpaired: HashMap<usize, u32> = HashMap::new();
    let mut sum = 0;

    let mut cursor = 0;
    while cursor < input.len() {
        let mut current = cursor;
        unsafe {
            while input.get_unchecked(current).is_ascii_digit() {
                current += 1;
            }
        }
        if current == cursor {
            current += 1;
        } else if let Some(index) =
            find_adjacent_symbol(Some(b'*'), symbols, row_length, cursor, current)
        {
            let num: u32 = std::str::from_utf8(&input[cursor..current])
                .expect("a string")
                .parse()
                .expect("a number");

            match unpaired.get(&index) {
                Some(value) => {
                    sum += value * num;
                }
                None => {
                    unpaired.insert(index, num);
                }
            }
        }
        cursor = current;
    }

    println!("{:?}", sum);
}

fn main() {
    let input = include_str!("../input").as_bytes();
    let row_length = input
        .iter()
        .position(|&c| c == b'\n')
        .expect("a newline is found")
        + 1;
    let symbols: HashMap<usize, u8> = input
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| {
            if c == b'\n' || c == b'.' || c.is_ascii_digit() {
                None
            } else {
                Some((index, c))
            }
        })
        .collect();

    part1(input, row_length, &symbols);
    part2(input, row_length, &symbols);
}
