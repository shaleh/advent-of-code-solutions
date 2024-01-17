use advent_support::read_input;

#[derive(Clone, Debug)]
enum Terminal {
    Command(String, Option<String>),
    Dir(String),
    File(usize, String),
}

fn parse_line(input: &str) -> Terminal {
    if input.starts_with('$') {
        match &input[2..4] {
            "cd" => Terminal::Command("cd".to_string(), Some(input[5..].to_string())),
            "ls" => Terminal::Command("ls".to_string(), None),
            _ => {
                unreachable!();
            }
        }
    } else if input.starts_with("dir ") {
        Terminal::Dir(input[4..].to_string())
    } else {
        let (size, name) = input.split_once(' ').unwrap();
        Terminal::File(size.parse().unwrap(), name.to_string())
    }
}

fn process_input(input: Vec<String>) -> Vec<Terminal> {
    input.into_iter().map(|line| parse_line(&line)).collect()
}

fn part_one(input: &[Terminal]) {
    dbg!(input);
    let solution = 0;
    println!("{}", solution);
}

// fn part_two() {
//     let solution = _;
//     println!("{}", solution);
// }

fn main() {
    let input = process_input(read_input().unwrap());

    part_one(&input);
    //part_two();
}
