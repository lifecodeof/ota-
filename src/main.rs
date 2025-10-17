mod ast;
mod codegen;
mod error_reporting;
mod lexer;
mod location;
mod parser;
mod semantic;
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

fn main() -> Result<(), crate::error_reporting::OtagError> {
    let args = Args::parse();

    let input = fs::read_to_string(&args.input_file).map_err(|e| crate::error_reporting::OtagError::runtime(format!("Dosya okuma hatası: {}", e), crate::location::Location::new(args.input_file.clone(), 0, 0)))?;

    // Parse
    let program = parser::parse(&input, &args.input_file)?;

    // Semantic analysis
    let mut analyzer = semantic::SemanticAnalyzer::new();
    analyzer.analyze_program(&program)?;

    // Execute
    let mut interpreter = codegen::Interpreter::new();
    interpreter.execute_program(&program).map_err(|e| crate::error_reporting::OtagError::runtime(e, crate::location::Location::unknown()))?;

    Ok(())
}
