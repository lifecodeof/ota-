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
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use ast::{Program, Statement};

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

/// Load a program with all its imports recursively
fn load_program_with_imports(
    file_path: &str,
    loaded_files: &mut HashSet<PathBuf>,
) -> Result<Program, crate::error_reporting::OtagError> {
    // Canonicalize the path to handle relative paths and prevent duplicate loads
    let abs_path = PathBuf::from(file_path)
        .canonicalize()
        .map_err(|e| {
            crate::error_reporting::OtagError::runtime(
                format!("Dosya yolu çözümlenemedi: {}", e),
                crate::location::Location::new(file_path.to_string(), 0, 0),
            )
        })?;

    // Check for circular imports
    if loaded_files.contains(&abs_path) {
        // Already loaded, return empty program to avoid duplication
        return Ok(Program {
            statements: Vec::new(),
        });
    }

    // Mark this file as loaded
    loaded_files.insert(abs_path.clone());

    // Read and parse the file
    let input = fs::read_to_string(&abs_path).map_err(|e| {
        crate::error_reporting::OtagError::runtime(
            format!("Dosya okuma hatası: {}", e),
            crate::location::Location::new(file_path.to_string(), 0, 0),
        )
    })?;

    let mut program = parser::parse(&input, file_path)?;

    // Process imports
    let mut all_statements = Vec::new();
    let base_dir = abs_path.parent().unwrap_or(Path::new("."));

    for statement in program.statements {
        if let Statement::Import(import_stmt) = &statement {
            // Resolve the import path relative to the current file
            let import_path = base_dir.join(&import_stmt.path);
            let import_path_str = import_path
                .to_str()
                .ok_or_else(|| {
                    crate::error_reporting::OtagError::runtime(
                        format!("Geçersiz içe aktarma yolu: {}", import_stmt.path),
                        crate::location::Location::unknown(),
                    )
                })?;

            // Recursively load the imported module
            let imported_program = load_program_with_imports(import_path_str, loaded_files)?;

            // Add all statements from the imported program
            all_statements.extend(imported_program.statements);
        } else {
            // Add non-import statements
            all_statements.push(statement);
        }
    }

    Ok(Program {
        statements: all_statements,
    })
}

fn main() -> Result<(), crate::error_reporting::OtagError> {
    let args = Args::parse();

    // Load program with all imports
    let mut loaded_files = HashSet::new();
    let program = load_program_with_imports(&args.input_file, &mut loaded_files)?;

    // Semantic analysis
    let mut analyzer = semantic::SemanticAnalyzer::new();
    analyzer.analyze_program(&program)?;

    // Execute
    let mut interpreter = codegen::Interpreter::new();
    interpreter.execute_program(&program).map_err(|e| crate::error_reporting::OtagError::runtime(e, crate::location::Location::unknown()))?;

    Ok(())
}
