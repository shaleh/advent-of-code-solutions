use std::collections::HashMap;

use advent_support::read_input;

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

fn main() {
    let lines = read_input::<String>().expect("Invalid input");

    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    let mut delivery_agents = vec![DeliveryAgent::new(), DeliveryAgent::new()];

    let entry = houses.entry(delivery_agents[0].position()).or_insert(0);
    *entry += 2;

    let mut which_agent = 0;
    for line in lines {
        for c in line.chars() {
            delivery_agents[which_agent].change_position(c);
            let entry = houses
                .entry(delivery_agents[which_agent].position())
                .or_insert(0);
            *entry += 1;
            which_agent ^= 1;
        }
    }
    dbg!(&houses);
    println!("Houses: {}", houses.len());
}
