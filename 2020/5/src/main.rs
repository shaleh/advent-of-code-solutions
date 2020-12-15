use std::cmp::max;
use std::io::{self, BufRead};

fn read_input() -> Vec<String> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.lines().map(|x| x.unwrap()).collect()
}

fn parse_value(value: &str, max: u32, lower: char, upper: char) -> u32 {
    let mut pos = 0;
    let mut current_max = max;
    for c in value.chars() {
        //dbg!(c, pos, current_max);
        if c == lower {
            current_max = ((current_max - pos) / 2) + pos;
        } else if c == upper {
            pos += (current_max - pos) / 2;
        }
    }
    pos
}

fn parse_row_and_column(value: &str) -> (u32, u32) {
    let row_info = &value[0..7];
    let column_info = &value[7..10];
    dbg!(row_info, column_info);

    (parse_value(row_info, 128, 'F', 'B'), parse_value(column_info, 8, 'L', 'R'))
}

const fn compute_seat_id(row: u32, column: u32) -> u32 {
    (row * 8) + column
}

fn main() {
    let data = read_input();
    let mut max_id = 0;
    for line in data {
        let (row, column) = parse_row_and_column(&line);
        let seat_id = compute_seat_id(row, column);
        max_id = max(max_id, seat_id);
        println!("row {}, column {}, id {}", row, column, seat_id);
    }
    println!("Max id: {}", max_id);
}
