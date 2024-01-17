use std::collections::HashMap;

use advent_support::read_input;

fn part_one(input: &[String]) {
    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    let mut current_x = 0;
    let mut current_y = 0;
    houses.entry((current_x, current_y)).or_insert(1);

    for line in input {
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

    println!("Houses: {}", houses.len());
}

struct DeliveryAgent {
    x: i64,
    y: i64,
}

impl DeliveryAgent {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn change_position(&mut self, direction: char) {
        match direction {
            '^' => {
                self.y += 1;
            }
            '>' => {
                self.x += 1;
            }
            'v' => {
                self.y += -1;
            }
            '<' => {
                self.x += -1;
            }
            _ => {
                panic!("Invalid input");
            }
        }
    }

    fn position(&self) -> (i64, i64) {
        return (self.x, self.y);
    }
}

fn part_two(input: &[String]) {
    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    let mut delivery_agents = vec![DeliveryAgent::new(), DeliveryAgent::new()];

    let entry = houses.entry(delivery_agents[0].position()).or_insert(0);
    *entry += 2;

    let mut which_agent = 0;
    for line in input {
        for c in line.chars() {
            delivery_agents[which_agent].change_position(c);
            let entry = houses
                .entry(delivery_agents[which_agent].position())
                .or_insert(0);
            *entry += 1;
            which_agent ^= 1;
        }
    }

    println!("Houses: {}", houses.len());
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    part_one(&lines);
    part_two(&lines);
}
