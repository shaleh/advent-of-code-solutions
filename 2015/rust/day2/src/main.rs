use advent_support::read_input;

fn paper_needed(sides: &[i32]) -> i64 {
    let length: i64 = sides[0].into();
    let width: i64 = sides[1].into();
    let height: i64 = sides[2].into();

    let smallest: i64 = length * width;

    (2 * length * width) + (2 * width * height) + (2 * height * length) + smallest
}

fn ribbon_needed(sides: &[i32]) -> i64 {
    let length: i64 = sides[0].into();
    let width: i64 = sides[1].into();
    let height: i64 = sides[2].into();

    let mut pairs = vec![(length, width), (length, height), (width, height)];
    pairs.sort();

    (length * width * height) + (2 * pairs[0].0) + (2 * pairs[0].1)
}

fn part_one(input: &[String]) {
    let total_paper_needed: i64 = input.iter().map(|line| {
        let mut sides: Vec<i32> = line.split('x').map(|v| v.parse::<i32>().unwrap()).collect();
        sides.sort();
        sides
    }).map(|sides| paper_needed(&sides)).sum();
    println!("Paper needed: {}", total_paper_needed);
}

fn part_two(input: &[String]) {
    let total_ribbon_needed: i64 = input.iter().map(|line| {
        let mut sides: Vec<i32> = line.split('x').map(|v| v.parse::<i32>().unwrap()).collect();
        sides.sort();
        sides
    }).map(|sides| ribbon_needed(&sides)).sum();
    println!("Ribbon needed: {}", total_ribbon_needed);
}

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    part_one(&lines);
    part_two(&lines);
}
