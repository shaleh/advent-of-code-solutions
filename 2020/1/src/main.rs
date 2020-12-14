use std::convert::TryInto;
use std::env;
use std::io::{self, BufRead};
use itertools::Itertools;

fn get_when_sum(values: &[i32], number_of_elements: u32, target: i32) -> Option<Vec<i32>> {
    let it = values.iter().cloned();
    let perms = it.permutations(number_of_elements.try_into().unwrap());
    perms.filter(|p| p.iter().sum::<i32>() == target).take(1).next()
}

fn read_input() -> Vec<i32> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (number_of_elements, desired_sum) = match args.len() {
        2 => (2, args[1].parse::<i32>().unwrap()),
        3 => (args[1].parse::<u32>().unwrap(), args[2].parse::<i32>().unwrap()),
        _ => {
            panic!("Usage: need number of elements and expected sum");
        }
    };
    let numbers = read_input();
    match get_when_sum(&numbers, number_of_elements, desired_sum) {
        Some(v) => { println!("{:?} = {}", v, v.iter().product::<i32>()); }
        _ => { println!("Not found"); }
    }
    println!("done");
}
