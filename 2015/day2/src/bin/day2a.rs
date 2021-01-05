use advent_support::read_input;

fn paper_needed(sides: &[i32]) -> i64 {
    let length: i64 = sides[0].into();
    let width: i64 = sides[1].into();
    let height: i64 = sides[2].into();

    let smallest: i64 = length * width;

    (2 * length * width) + (2 * width * height) + (2 * height * length) + smallest
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    let mut total_paper_needed = 0;
    for line in lines {
        let mut sides: Vec<i32> = line.split('x').map(|v| v.parse::<i32>().unwrap()).collect();
        sides.sort();
        total_paper_needed += paper_needed(&sides);
    }
    println!("Paper needed: {}", total_paper_needed);
}
