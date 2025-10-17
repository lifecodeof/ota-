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
    
    // Parse
    let program = parser::parse(&input)?;
    
    // Generate code
    let mut codegen = codegen::CodeGen::new();
    codegen.generate_program(&program)?;
    
    Ok(())
}
