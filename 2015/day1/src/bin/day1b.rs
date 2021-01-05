use advent_support::read_input;

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    for line in lines {
        let mut current_floor = 0;
        let mut index = 0;
        for c in line.chars() {
            let value = match c {
                '(' => 1,
                ')' => -1,
                _ => { panic!("not handled"); },
            };
            current_floor += value;
            if current_floor == -1 {
                break;
            }
            index += 1;
        }
        // 1-based index.
        println!("Index {}", index + 1);
    }
}
