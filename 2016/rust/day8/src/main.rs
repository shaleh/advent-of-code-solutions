use std::fmt::{self, Display};

use regex::Regex;

struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen {
            buffer: vec![false; width * height],
            width,
            height,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    fn count(&self) -> usize {
        self.buffer.iter().filter(|x| **x).count()
    }

    fn fill(&mut self, fill_width: usize, fill_height: usize) {
        for y in 0..fill_height {
            for x in 0..fill_width {
                let index = self.index(x, y);
                self.buffer[index] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, count: usize) {
        let row: Vec<bool> = self
            .buffer
            .iter()
            .copied()
            .skip(y * self.width)
            .take(self.width)
            .collect();
        for (x, value) in row.into_iter().enumerate() {
            let new_x = (x + count) % self.width;
            let new_pos = self.index(new_x, y);
            self.buffer[new_pos] = value;
        }
    }

    fn rotate_column(&mut self, x: usize, count: usize) {
        let column: Vec<bool> = self
            .buffer
            .iter()
            .copied()
            .skip(x)
            .step_by(self.width)
            .collect();
        for (y, value) in column.into_iter().enumerate() {
            let new_y = (y + count) % self.height;
            let index = self.index(x, new_y);
            self.buffer[index] = value;
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".")?;
        for _ in 0..self.width {
            write!(f, "=")?;
        }
        writeln!(f, ".")?;
        for y in 0..self.height {
            write!(f, "|")?;
            for x in 0..self.width {
                let symbol = match self.buffer[self.index(x, y)] {
                    true => '#',
                    false => ' ',
                };
                write!(f, "{symbol}")?;
            }
            writeln!(f, "|")?;
        }
        write!(f, ".")?;
        for _ in 0..self.width {
            write!(f, "=")?;
        }
        write!(f, ".")?;

        Ok(())
    }
}

fn main() {
    let data = include_str!("../input");

    let rect_re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_re = Regex::new(r"rotate (column|row) (?:x|y)=(\d+) by (\d+)").unwrap();

    let mut screen = Screen::new(50, 6);

    let operations: Vec<_> = data.lines().collect();
    for (idx, op) in operations.into_iter().enumerate() {
        println!("{}: {:?}", idx, op);
        if let Some(captures) = rect_re.captures(op) {
            let fill_width = (captures[1]).parse().unwrap();
            let fill_height = (captures[2]).parse().unwrap();
            screen.fill(fill_width, fill_height);
        } else if let Some(captures) = rotate_re.captures(op) {
            let count: usize = (captures[3]).parse().unwrap();

            match &captures[1] {
                "row" => {
                    let y: usize = (captures[2]).parse().unwrap();
                    screen.rotate_row(y, count);
                }
                "column" => {
                    let x: usize = (captures[2]).parse().unwrap();
                    screen.rotate_column(x, count);
                }
                _ => unreachable!(),
            }
        } else {
            panic!("No match {op}");
        }
    }

    println!("{screen}");
    println!("{}", screen.count());
}

#[test]
fn tests() {
    assert_eq!(index(50, 0, 0), 0);
    assert_eq!(index(50, 49, 0), 49);
    assert_eq!(index(50, 0, 1), 50);
    assert_eq!(index(50, 4, 1), 54);
}
