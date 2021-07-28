use std::collections::HashMap;

#[macro_use]
extern crate pest_derive;

use anyhow::Result;

mod wires;
use wires::{eval, load_wire_connections, Operation};

fn main() -> Result<()> {
    let mut wire_connections: HashMap<String, Operation> = HashMap::new();

    load_wire_connections(&mut wire_connections)?;

    let mut wire_state: HashMap<String, u16> = HashMap::new();
    eval(&wire_connections, &mut wire_state, "a")?;
    let a_value = wire_state.get("a").unwrap();
    println!("{:?}", a_value);

    Ok(())
}
