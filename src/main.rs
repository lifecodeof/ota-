mod ast;
mod codegen;
mod lexer;
mod parser;
mod symbol_table;
mod types;

use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(name = "otağ")]
#[command(version = "0.1.0")]
#[command(about = "Otağ Programming Language Compiler")]
#[command(long_about = "A Turkish-localized programming language compiler that supports variable declarations, expressions, and output statements.")]
struct Args {
    /// Input Otağ source file (.otağ)
    #[arg(value_name = "FILE")]
    input_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input = fs::read_to_string(&args.input_file)?;

    // Parse
    let program = parser::parse(&input)?;

    // Execute
    let mut interpreter = codegen::Interpreter::new();
    interpreter.execute_program(&program)?;

    Ok(())
}
