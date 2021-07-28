use advent_support::read_input;

struct Floor(i64);

impl Floor {
    fn new() -> Self {
        Self(0)
    }

    fn up(&mut self) -> Option<i64> {
        self.0 += 1;
        Some(self.0)
    }

    fn down(&mut self) -> Option<i64> {
        if self.0 > 0 {
            self.0 -= 1;
            Some(self.0)
        } else {
            None
        }
    }
}

fn compute_index(input: &str) -> i64 {
    let mut floor = Floor::new();

    let mut index = 0 as usize;
    for (idx, c) in input.chars().enumerate() {
        let result = match c {
            '(' => floor.up(),
            ')' => floor.down(),
            _ => None,
        };
        match result {
            None => {
                return idx as i64;
            }
            Some(_) => {
                index = idx;
            }
        }
    }
    index as i64
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    for line in lines {
        let index = compute_index(&line);
        // 1-based index.
        println!("Index {}", index + 1);
    }
}
