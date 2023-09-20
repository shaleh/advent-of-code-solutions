mod instruction;
mod parse;

use crate::instruction::Instruction;
use crate::parse::parse;

fn new_index(index: usize, offset: i64) -> Result<usize, String> {
    let result = if offset < 0 {
        index.checked_sub((-offset) as usize)
    } else {
        index.checked_add(offset as usize)
    };
    result.ok_or(format!(
        "Invalid offset for jump at index {}, offset {}",
        index, offset
    ))
}

fn evaluate(
    instructions: &[Instruction],
    initial_a: u64,
    initial_b: u64,
) -> Result<[u64; 2], String> {
    let mut index = 0;
    let mut registers = [initial_a, initial_b];

    while index < instructions.len() {
        match &instructions[index] {
            Instruction::Hlf(register) => {
                let current = registers[*register];
                registers[*register] = current / 2;
            }
            Instruction::Tpl(register) => {
                let current = registers[*register];
                registers[*register] = current * 3;
            }
            Instruction::Inc(register) => {
                let current = registers[*register];
                registers[*register] = current + 1;
            }
            Instruction::Jmp(offset) => {
                index = new_index(index, *offset)?;
                continue;
            }
            Instruction::Jie((register, offset)) => {
                if registers[*register] % 2 == 0 {
                    index = new_index(index, *offset)?;
                    continue;
                }
            }
            Instruction::Jio((register, offset)) => {
                if registers[*register] == 1 {
                    index = new_index(index, *offset)?;
                    continue;
                }
            }
        }

        index += 1;
    }

    Ok(registers)
}

fn main() {
    let data = include_str!("../input");
    let instructions: Vec<Instruction> = parse(data);
    match evaluate(&instructions, 0, 0) {
        Ok(registers) => {
            println!("a = {}, b = {}", registers[0], registers[1]);
        }
        Err(msg) => {
            println!("Error: {}", msg);
        }
    }

    match evaluate(&instructions, 1, 0) {
        Ok(registers) => {
            println!("a = {}, b = {}", registers[0], registers[1]);
        }
        Err(msg) => {
            println!("Error: {}", msg);
        }
    }
}
