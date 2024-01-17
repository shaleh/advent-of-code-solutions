use std::fmt::{self, Display};

use regex::Regex;

struct Screen {
    width: usize,
    height: usize,
    backend1: Vec<bool>,
    backend2: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            backend1: vec![false; width * height],
            backend2: vec![false; width * height],
            width,
            height,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        (x * self.height) + y
    }

    fn count(&self) -> usize {
        self.backend1.iter().filter(|x| **x).count()
    }

    fn fill(&mut self, width: usize, height: usize) {
        for x in 0..width {
            for y in 0..height {
                self.backend1[self.index(x, y)] = true;
            }
        }
    }

    fn rotate_row(&self, y: usize, count: usize) {
        for x in 0..self.width {
            let new_x = (x + count) % self.width;
            self.backend1[self.index(new_x, y)] = self.backend2[self.index(x, y)];
        }
    }

    fn rotate_column(&self, x: usize, count: usize) {
        for y in 0..self.height {
            let new_y = (y + count) % self.height;
            self.backend1[self.index(x, new_y)] = self.backend2[self.index(x, y)];
        }
    }

    fn save(&mut self) {
        self.backend2.copy_from_slice(&self.backend1)
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.width {
            for y in 0..self.height {
                let symbol = match self.backend1[self.index(x, y)] {
                    true => '#',
                    false => '.',
                };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let data = include_str!("../input");

    let rect_re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_re = Regex::new(r"rotate (column|row) (?:x|y)=(\d+) by (\d+)").unwrap();

    let width = 50;
    let height = 6;
    let mut screen = Screen::new(width, height);

    let operations: Vec<_> = data.lines().collect();
    for (idx, op) in operations.into_iter().enumerate() {
        if let Some(captures) = rect_re.captures(op) {
            let fill_width = (&captures[1]).parse().unwrap();
            let fill_height = (&captures[2]).parse().unwrap();
            screen.fill(fill_width, fill_height);
        } else if let Some(captures) = rotate_re.captures(op) {
            let count: usize = (&captures[3]).parse().unwrap();

            match &captures[1] {
                "row" => {
                    let y: usize = (&captures[2]).parse().unwrap();
                    screen.rotate_row(y, count);
                }
                "column" => {
                    let x: usize = (&captures[2]).parse().unwrap();
                    screen.rotate_column(x, count);
                }
                _ => unreachable!(),
            }
        } else {
            panic!("No match {op}");
        }

        screen.save();

        if idx == 2 {
            break;
        }
    }

    println!("{screen}");
    println!("{}", screen.count());
}
