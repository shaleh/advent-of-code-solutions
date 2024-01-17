use std::fmt::Debug;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn read_input<T: FromStr>() -> Vec<T>
where <T as FromStr>::Err: Debug
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut results = Vec::<T>::new();
    for line in handle.lines() {
        let line = line.unwrap();
        let value = line.parse::<T>().unwrap();
        results.push(value);
    }

    results
}

fn main() {
    let input = read_input();
    let mut previous: Option<i32> = None;
    let mut increasing = 0;
    for depth in input {
        if let Some(p) = previous {
            if depth > p {
                increasing += 1;
            }
        }
        previous = Some(depth);
    }
    println!("{}", increasing);
}
