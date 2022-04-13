mod vm;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n*************** Turing-Sim ***************\n");

    let path = "test.tm";
    let code = fs::read(path).expect("Could not read code from file");

    vm::interpret(code).expect("Execution unsuccessful");

    println!("\n******************************************\n");

    Ok(())
}
