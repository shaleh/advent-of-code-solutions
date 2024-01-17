fn compute_code(row: u64, column: u64, initial: u64) -> u64 {
    let multiplier = 252533;
    let divisor = 33554393;

    let value = (row + column) - 1;
    let target = (((value * value) + value) / 2) - (row - 1);
    (1..target).fold(initial, |previous, _| (previous * multiplier) % divisor)
}

fn main() {
    let initial = 20151125;
    let goal = compute_code(3010, 3019, initial);
    println!("{goal:?}");
}
