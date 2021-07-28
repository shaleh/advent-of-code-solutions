use advent_support::read_input;

struct Floor(i64);

impl Floor {
    fn new() -> Self {
        Self(0)
    }

    fn up(&mut self) {
        self.0 += 1;
    }

    fn down(&mut self) {
        self.0 -= 1;
    }

    fn value(&self) -> i64 {
        self.0
    }
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    for line in lines {
        let mut floor = Floor::new();
        line.chars().for_each(|c| match c {
            '(' => floor.up(),
            ')' => floor.down(),
            _ => {
                panic!("not handled");
            }
        });
        println!("Final floor: {}", floor.value());
    }
}
