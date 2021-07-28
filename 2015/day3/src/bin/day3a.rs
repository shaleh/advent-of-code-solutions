use std::collections::HashMap;

use advent_support::read_input;

fn main() {
    let lines = read_input::<String>().expect("Invalid input");

    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    let mut current_x = 0;
    let mut current_y = 0;
    houses.entry((current_x, current_y)).or_insert(1);

    for line in lines {
        for c in line.chars() {
            match c {
                '^' => {
                    current_y += 1;
                }
                '>' => {
                    current_x += 1;
                }
                'v' => {
                    current_y += -1;
                }
                '<' => {
                    current_x += -1;
                }
                _ => {
                    panic!("Invalid input");
                }
            }
            let entry = houses.entry((current_x, current_y)).or_insert(0);
            *entry += 1;
        }
    }
    dbg!(&houses);
    println!("Houses: {}", houses.len());
}
