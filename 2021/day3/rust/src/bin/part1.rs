use std::io::{self, BufRead};

pub fn read_input() -> Vec<Vec<i8>>
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    handle.lines().map(|line| {
        let line = line.unwrap();
        line.chars().map(|b| (b as i8) - '0' as i8).collect()
    }).collect()
}

/*
for each binary
   for each bit
     increment either 1 or 0 count for this bit

examine bit counts
for each bit
    if 1 > 0
      put 1 on gamma, 0 on epsilon
    else
      put 0 on gamma, 1 on epsilon
 */
fn main() {
    let input = read_input();

    let mut ones = [0u32; 12];
    let mut zeroes = [0u32; 12];

    for bits in input {
        for (index, bit) in bits.iter().enumerate() {
            match bit {
                0 => { zeroes[index] += 1; },
                1 => { ones[index] += 1; },
                _ => { unreachable!(); },
            }
        }
    }
    println!("{:?}, {:?}", ones, zeroes);

    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    
    for index in 0..12 {
        if ones[index] > zeroes[index] {
            gamma |= 1 << index;
        } else {
            epsilon |= 1 << index;
        }
    }

    println!("gamma: {}, epsilon {}, energy: {}", gamma, epsilon, gamma * epsilon);
}
