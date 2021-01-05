use advent_support::read_input;

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    for line in lines {
        let result: i64 = line.chars().map(|c| 
            match c {
                '(' => 1,
                ')' => -1,
                _ => { panic!("not handled"); },
            }).sum();
        println!("Final floor: {}", result);
    }
}
