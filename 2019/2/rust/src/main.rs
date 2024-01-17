#![feature(drain_filter)]
#![feature(result_map_or_else)]
#![feature(slice_patterns)]

use std::io::{self, BufRead};

fn parse(input: String) -> Vec<u32> {
    println!("{:?}", input);
    let code = input.split(",").filter(|x| !x.is_empty()).map(|x| {
        x.to_string().parse::<u32>().map_or_else(
            |e| {
                println!("Error: {}: {}", e, x);
                ::std::process::exit(1);
            },
            |v| v,
        )
    });
    code.collect::<Vec<u32>>()
}

fn evaluate_bin_op(
    op: u32,
    param1: u32,
    param2: u32,
    result_address: u32,
    code: &mut Vec<u32>,
) -> Result<u32, String> {
    code[result_address as usize] = match op {
        1 => param1 + param2,
        2 => param1 * param2,
        _ => return Err(format!("unknown operation: {:?}", op)),
    };
    Ok(4)
}

fn run(code: Vec<u32>) -> Vec<u32> {
    let mut instruction_pointer: usize = 0;
    let mut memory = code.clone();

    loop {
        let increment = match memory[instruction_pointer] {
            99 => break,
            1 | 2 => match &memory[instruction_pointer..instruction_pointer + 4] {
                [op, param1, param2, result_address] => {
                    evaluate_bin_op(*op, *param1, *param2, *result_address, &mut memory)
                        .unwrap_or_else(|e| {
                            println!("Error: {} @ {}", e, instruction_pointer);
                            ::std::process::exit(1);
                        })
                }
                _ => {
                    println!("Error: invalid instruction: {:?}", &memory[0..4]);
                    ::std::process::exit(1);
                }
            },
            _ => {
                println!(
                    "Error: unknown opcode: {} @ {}",
                    code[instruction_pointer as usize], instruction_pointer
                );
                ::std::process::exit(1);
            }
        };
        instruction_pointer += increment as usize;
    }

    memory
}

fn main() -> io::Result<()> {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("");
    let code = parse(input);
    println!("{:?}", code);
    println!("{:?}", run(code));
    // buffer = buffer.trim().to_string();
    // println!("{}", buffer);
    // let value = buffer.parse::<i32>().unwrap();
    // println!("{}", value / 3 - 2);

    Ok(())
}
