use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
	match self {
	    Self::North => Self::West,
	    Self::East  => Self::North,
	    Self::South => Self::East,
	    Self::West  => Self::South,
	}
    }

    fn turn_right(&self) -> Direction {
	match self {
	    Self::North => Self::East,
	    Self::East  => Self::South,
	    Self::South => Self::West,
	    Self::West  => Self::North,
	}
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn shift(&self, direction: Direction, count: i64) -> Position {
	match direction {
	    Direction::North => Position { y: self.y + count, ..*self },
	    Direction::South => Position { y: self.y - count, ..*self },
	    Direction::East => Position { x: self.x + count, ..*self },
	    Direction::West => Position { x: self.x - count, ..*self },
	}
    }
}

fn walk(steps: &[&str], stop_on_second_visit: bool) -> (Direction, Position) {
    let mut position = Position { x: 0, y: 0 };
    let mut direction = Direction::North;
    let mut seen_locations = HashSet::new();
    seen_locations.insert(position);

    for step in steps {
	//println!("{step}");
	let mut chars = step.chars();
	let turn = chars.by_ref().take(1).next().unwrap();
	let count = chars.collect::<String>().parse().unwrap();
	//println!("{position:?} {direction:?} {turn} {count}");
	direction = match turn {
	    'L' => direction.turn_left(),
	    'R' => direction.turn_right(),
	    _ => unreachable!(),
	};

	for _ in 0..count {
	    position = position.shift(direction, 1);
	    if stop_on_second_visit && !seen_locations.insert(position) {
		return (direction, position);
	    }
	}
    }

    (direction, position)
}

fn main() {
    let input = include_str!("../input");w
    let steps: Vec<_> = input.split(", ").collect();

    let (direction, position) = walk(&steps, false);
    println!("{direction:?}, {position:?}");
    println!("{}", position.x + position.y);

    let (direction, position) = walk(&steps, true);
    println!("{direction:?}, {position:?}");
    println!("{}", position.x + position.y);
}
