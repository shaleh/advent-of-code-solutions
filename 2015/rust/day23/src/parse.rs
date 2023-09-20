use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace1, one_of};
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

use crate::instruction::Instruction;

fn map_register(name: char) -> usize {
    match name {
        'a' => 0,
        'b' => 1,
        _ => unreachable!(),
    }
}

fn offset(input: &str) -> IResult<&str, i64> {
    map_res(recognize(preceded(opt(one_of("-+")), digit1)), |s: &str| {
        s.parse()
    })(input)
}

fn with_register(input: &str) -> IResult<&str, Instruction> {
    let (input, (name, register)) = separated_pair(
        alt((tag("hlf"), tag("inc"), tag("tpl"))),
        multispace1,
        one_of("ab"),
    )(input)?;

    let register_index = map_register(register);
    let keyword = match name {
        "hlf" => Instruction::Hlf,
        "inc" => Instruction::Inc,
        "tpl" => Instruction::Tpl,
        _ => unreachable!(),
    };
    Ok((input, keyword(register_index)))
}

fn jump(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, value)) = separated_pair(tag("jmp"), multispace1, offset)(input)?;
    Ok((input, Instruction::Jmp(value)))
}

fn jump_if(input: &str) -> IResult<&str, Instruction> {
    let (input, name) = alt((tag("jie"), tag("jio")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, register) = one_of("ab")(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = multispace1(input)?;
    let (input, offset) = offset(input)?;

    let register_index = map_register(register);

    let keyword = match name {
        "jie" => Instruction::Jie,
        "jio" => Instruction::Jio,
        _ => unreachable!(),
    };
    Ok((input, keyword((register_index, offset))))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((with_register, jump, jump_if))(input)
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let (instructions, errors): (Vec<_>, Vec<_>) =
        input.lines().map(instruction).partition(|x| x.is_ok());
    if !errors.is_empty() {
        panic!("Errors: {:?}", errors);
    }
    let instructions: Vec<_> = instructions
        .into_iter()
        .map(|x| x.unwrap())
        .map(|x| x.1)
        .collect();
    instructions
}
