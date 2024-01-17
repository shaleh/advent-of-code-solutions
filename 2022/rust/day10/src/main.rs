use std::fs;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum CpuOpcode {
    Noop,
    Addx(i64),
}

impl FromStr for CpuOpcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<_> = s.split(' ').collect();
        match pieces.as_slice() {
            ["noop"] => Ok(Self::Noop),
            ["addx", num] => Ok(Self::Addx(num.parse::<i64>().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct CpuState {
    accumulator: i64,
    register: i64,
    cycle: usize,
    current_op: Option<CpuOpcode>,
    should_render: bool,
    screen: Vec<Vec<char>>,
}

impl CpuState {
    fn new(should_render: bool) -> Self {
        Self {
            accumulator: 0,
            register: 1,
            cycle: 0,
            current_op: None,
            should_render,
            screen: vec![
                vec!['.'; 40],
                vec!['.'; 40],
                vec!['.'; 40],
                vec!['.'; 40],
                vec!['.'; 40],
                vec!['.'; 40],
            ],
        }
    }

    fn evaluate(&mut self, opcodes: &[CpuOpcode]) -> i64 {
        let mut codes: Vec<_> = opcodes.iter().rev().collect();
        while !codes.is_empty() {
            self.cycle += 1;

            if [20, 60, 100, 140, 180, 220].contains(&self.cycle) {
                self.accumulator += self.cycle as i64 * self.register;
            }

            if self.should_render {
                self.render();
            }

            match self.current_op {
                Some(CpuOpcode::Addx(value)) => {
                    self.register += value;
                    self.current_op = None;
                }
                Some(op) => {
                    unreachable!("unknown op: {:?}", op);
                }
                None => match codes.pop().expect("next op code") {
                    CpuOpcode::Noop => {}
                    op => {
                        self.current_op = Some(*op);
                    }
                },
            }
        }

        self.accumulator
    }

    fn render(&mut self) {
        let position = (self.cycle - 1) % 40;
        let pixel = if ((self.register - 1)..=(self.register + 1)).contains(&(position as i64)) {
            '#'
        } else {
            '.'
        };
        self.screen[(self.cycle - 1) / 40][position] = pixel;

        print!("{}", pixel);
        if self.cycle % 40 == 0 {
            println!();
        }
    }
}

fn solve(opcodes: &[CpuOpcode], should_render: bool) -> i64 {
    let mut cpu = CpuState::new(should_render);

    cpu.evaluate(opcodes)
}

fn part1(opcodes: &[CpuOpcode]) -> i64 {
    solve(opcodes, false)
}

fn part2(opcodes: &[CpuOpcode]) {
    solve(opcodes, true);
}

fn parse_input(input: &str) -> Vec<CpuOpcode> {
    input
        .lines()
        .map(|input| input.parse::<CpuOpcode>().expect("valid opcode"))
        .collect()
}

fn main() {
    let input_raw = fs::read_to_string("../../inputs/10").expect("data");
    let opcodes = parse_input(&input_raw);

    println!("part1: {}", part1(&opcodes));
    part2(&opcodes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = include_str!("example.input");
        let opcodes = parse_input(data);

        let mut cpu = CpuState::new(true);

        assert_eq!(cpu.evaluate(&opcodes), 13140);

        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];

        let result: Vec<String> = cpu.screen.iter().map(|row| row.iter().collect()).collect();
        assert_eq!(expected, result);
    }
}
