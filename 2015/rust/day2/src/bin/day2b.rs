use advent_support::read_input;

fn ribbon_needed(sides: &[i32]) -> i64 {
    let length: i64 = sides[0].into();
    let width: i64 = sides[1].into();
    let height: i64 = sides[2].into();

    let mut pairs = vec![(length, width), (length, height), (width, height)];
    pairs.sort();

    (length * width * height) + (2 * pairs[0].0) + (2 * pairs[0].1)
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    let mut total_ribbon_needed = 0;
    for line in lines {
        let mut sides: Vec<i32> = line.split('x').map(|v| v.parse::<i32>().unwrap()).collect();
        sides.sort();
        total_ribbon_needed += ribbon_needed(&sides);
    }
    println!("Ribbon needed: {}", total_ribbon_needed);
}
