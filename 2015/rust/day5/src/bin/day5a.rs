use advent_support::read_input;

fn is_nice(input: &str) -> bool {
    let mut prev = None;
    let mut vowels_count = 0;
    let mut doubles_count = 0;

    let exclusions = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

    for c in input.chars() {
        if let Some(prev_char) = prev {
            let couple = (prev_char, c);
            if exclusions.contains(&couple) {
                return false;
            }
            if prev_char == c {
                doubles_count += 1;
            }
        }

        if "aeiou".contains(c) {
            vowels_count += 1;
        }

        prev = Some(c);
    }

    vowels_count >= 3 && doubles_count > 0
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");

    let nice = lines.iter().filter(|s| is_nice(&s)).count();
    println!("{}", nice);
}
