mod types;
mod ast;
mod lexer;
mod parser;
mod codegen;
mod symbol_table;

use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input.otag>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1])?;
    eprintln!("Input: {:?}", input);
    
    // Parse
    let program = parser::parse(&input)?;
    eprintln!("Parsed program with {} statements", program.statements.len());
    
    // Execute
    let mut interpreter = codegen::Interpreter::new();
    interpreter.execute_program(&program)?;
    
    Ok(())
}
