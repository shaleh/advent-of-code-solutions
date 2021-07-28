use std::collections::HashMap;

use advent_support::read_input;

use anyhow::Result;
use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "wireA.pest"]
pub struct Wire;

#[derive(Clone, Debug)]
pub enum NameOrNumber {
    Name(String),
    Number(u16),
}

impl Into<String> for NameOrNumber {
    fn into(self) -> String {
        match self {
            NameOrNumber::Name(name) => name,
            NameOrNumber::Number(_) => unimplemented!("string from number"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    EVAL,
    NOT,
}

#[derive(Clone, Debug)]
pub enum Operation {
    BinaryOperation((BinaryOp, NameOrNumber, NameOrNumber)),
    UnaryOperation((UnaryOp, NameOrNumber)),
}

/*
  Wire state. A map from name to value.

  Parse line
    could be
     - signal to wire
     - binary op with 2 sources and a destination
     - unary op with 1 source and a destination

  Signal: Put value in wire name
  Binary op: Apply value from both wires to destination wire
  Unary op: apply value from wire to destination
*/
fn parse_signal(parser: Pair<Rule>) -> Result<Operation> {
    //println!("Signal {:?}", parser);
    let name_or_number = parse_name(parser.into_inner().next().expect("valid number"))?;
    Ok(Operation::UnaryOperation((UnaryOp::EVAL, name_or_number)))
}

fn parse_unary_operation(parser: Pair<Rule>) -> Result<Operation> {
    //println!("Unary op {:?}", parser);
    let mut inner = parser.into_inner();
    let op = match inner.next().expect("valid unary op").as_str() {
        "NOT" => UnaryOp::NOT,
        r @ _ => panic!("Unhandled Unary op {:?}", r),
    };
    let name_or_number = parse_name(inner.next().expect("a valid name or number"))?;
    Ok(Operation::UnaryOperation((op, name_or_number)))
}

fn parse_binary_operation(parser: Pair<Rule>) -> Result<Operation> {
    let mut inner = parser.into_inner();
    let name_or_number = inner
        .next()
        .expect("valid name or number")
        .into_inner()
        .next()
        .expect("parse valid name or number");
    let operand1 = parse_name(name_or_number)?;
    let op = match inner.next().expect("valid binary op").as_str() {
        "AND" => BinaryOp::AND,
        "OR" => BinaryOp::OR,
        "LSHIFT" => BinaryOp::LSHIFT,
        "RSHIFT" => BinaryOp::RSHIFT,
        r @ _ => panic!("Unhandled Unary op {:?}", r),
    };
    let name_or_number = inner
        .next()
        .expect("valid name or number")
        .into_inner()
        .next()
        .expect("parse valid name or number");
    let operand2 = parse_name(name_or_number)?;
    Ok(Operation::BinaryOperation((op, operand1, operand2)))
}

fn parse_operation(parser: Pair<Rule>) -> Result<Operation> {
    let mut inner = parser.into_inner();
    let op = inner.next().expect("valid op");
    match op.as_rule() {
        Rule::signal => parse_signal(op),
        Rule::unary_operation => parse_unary_operation(op),
        Rule::binary_operation => parse_binary_operation(op),
        r @ _ => {
            panic!("Unhandled rule {:?}", r);
        }
    }
}

fn parse_name(parser: Pair<Rule>) -> Result<NameOrNumber> {
    match parser.as_rule() {
        Rule::name => Ok(NameOrNumber::Name(parser.as_str().to_string())),
        Rule::number => {
            let number = parser.as_str().parse().expect("a number");
            Ok(NameOrNumber::Number(number))
        }
        Rule::name_or_number => parse_name(parser.into_inner().next().unwrap()),
        r @ _ => panic!("Unhandled rule as name: {:?}", r),
    }
}

pub fn parse(input: &str) -> Result<(Operation, String)> {
    let result = Wire::parse(Rule::circuit_part, input)?.next().unwrap();
    let mut circuit_parts = result.into_inner();
    let operation = circuit_parts.next().expect("valid operation");
    let op = parse_operation(operation)?;
    let _ = circuit_parts.next().expect("->");
    let name_part = circuit_parts.next().expect("valid name");
    let name = parse_name(name_part)?;

    Ok((op, name.into()))
}

fn eval_unary_op(
    op: UnaryOp,
    operand: NameOrNumber,
    state: &HashMap<String, u16>,
) -> Result<u16, String> {
    let value = match operand {
        NameOrNumber::Number(value) => value,
        NameOrNumber::Name(source) => match state.get(&source) {
            Some(value) => *value,
            None => {
                return Err(source);
            }
        },
    };

    match op {
        UnaryOp::EVAL => Ok(value),
        UnaryOp::NOT => Ok(!value),
    }
}

fn eval_binary_op(
    op: BinaryOp,
    operand1: NameOrNumber,
    operand2: NameOrNumber,
    state: &HashMap<String, u16>,
) -> Result<u16, (Option<String>, Option<String>)> {
    let (value1, source1) = match operand1 {
        NameOrNumber::Number(value) => (Some(value), None),
        NameOrNumber::Name(source) => match state.get(&source) {
            Some(value) => (Some(*value), None),
            None => (None, Some(source)),
        },
    };
    let (value2, source2) = match operand2 {
        NameOrNumber::Number(value) => (Some(value), None),
        NameOrNumber::Name(source) => match state.get(&source) {
            Some(value) => (Some(*value), None),
            None => (None, Some(source)),
        },
    };

    match (value1, value2) {
        (Some(v1), Some(v2)) => match op {
            BinaryOp::AND => Ok(v1 & v2),
            BinaryOp::OR => Ok(v1 | v2),
            BinaryOp::LSHIFT => {
                let (value, _) = v1.overflowing_shl(v2 as u32);
                Ok(value)
            }
            BinaryOp::RSHIFT => {
                let (value, _) = v1.overflowing_shr(v2 as u32);
                Ok(value)
            }
        },
        (_, _) => Err((source1, source2)),
    }
}

pub fn eval(
    connections: &HashMap<String, Operation>,
    state: &mut HashMap<String, u16>,
    wire: &str,
) -> Result<()> {
    let mut stack: Vec<String> = Vec::new();

    stack.push(wire.to_string());
    while let Some(name) = stack.last() {
        match connections.get(name).unwrap() {
            Operation::UnaryOperation((op, operand)) => {
                match eval_unary_op(*op, operand.clone(), &state) {
                    Ok(value) => {
                        state.insert(name.clone(), value);
                        stack.pop();
                    }
                    Err(source) => {
                        stack.push(source);
                    }
                }
            }
            Operation::BinaryOperation((op, operand1, operand2)) => {
                match eval_binary_op(*op, operand1.clone(), operand2.clone(), &state) {
                    Ok(value) => {
                        state.insert(name.clone(), value);
                        stack.pop();
                    }
                    Err((source1, source2)) => {
                        if let Some(source1) = source1 {
                            stack.push(source1);
                        }
                        if let Some(source2) = source2 {
                            stack.push(source2);
                        }
                    }
                }
            }
        }
    }

    // for (wire, value) in state.iter() {
    //     println!("{:?} <- {:?}", wire, value);
    // }

    Ok(())
}

pub fn load_wire_connections(connections: &mut HashMap<String, Operation>) -> Result<()> {
    let lines = read_input::<String>().expect("Invalid input");
    for line in lines {
        let (operation, wire_name) = parse(&line)?;
        // println!("{:?} {:?}", operation, wire_name);
        connections.insert(wire_name, operation);
    }

    Ok(())
}
