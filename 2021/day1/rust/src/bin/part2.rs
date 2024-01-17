use std::fmt::Debug;
use std::io::{self, BufRead};
use std::str::FromStr;

use itertools::multizip;

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
    let input: Vec<i32> = read_input();
    let mut previous: Option<i32> = None;
    let mut increasing = 0;
    for (d1, d2, d3) in multizip((input.iter(), input.clone().iter().skip(1), input.clone().iter().skip(2))) {
        println!("{} {} {}", d1, d2, d3);
        let sum = d1 + d2 + d3;
        if let Some(p) = previous {
            if sum > p {
                increasing += 1;
            }
        }
        previous = Some(sum);
    }
    println!("{}", increasing);
}
