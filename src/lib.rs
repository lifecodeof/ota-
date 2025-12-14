// Library module exposing internal API for testing and programmatic use

pub mod ast;
pub mod codegen;
pub mod error_reporting;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod semantic;
pub mod symbol_table;
pub mod types;

use ast::Program;
use error_reporting::{OtagError, Result};
use std::collections::HashMap;
use std::path::Path;

/// Represents a virtual file system for in-memory source files
pub struct VirtualFileSystem {
    files: HashMap<String, String>,
}

impl VirtualFileSystem {
    /// Create a new empty virtual file system
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Add a virtual file to the file system
    pub fn add_file(&mut self, path: impl Into<String>, content: impl Into<String>) {
        self.files.insert(path.into(), content.into());
    }

    /// Get the content of a virtual file
    pub fn get_file(&self, path: &str) -> Option<&str> {
        self.files.get(path).map(|s| s.as_str())
    }

    /// Check if a file exists
    pub fn file_exists(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }
}

impl Default for VirtualFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Otağ runtime environment for executing programs
pub struct OtagRuntime {
    vfs: VirtualFileSystem,
}

impl OtagRuntime {
    /// Create a new runtime with an empty virtual file system
    pub fn new() -> Self {
        Self {
            vfs: VirtualFileSystem::new(),
        }
    }

    /// Add an in-memory source file
    pub fn add_source(&mut self, path: impl Into<String>, content: impl Into<String>) {
        self.vfs.add_file(path, content);
    }

    /// Execute a program from an in-memory source file
    pub fn execute(&mut self, entry_file: &str) -> Result<()> {
        let program = self.load_program_with_imports(entry_file)?;

        // Semantic analysis
        let mut analyzer = semantic::SemanticAnalyzer::new();
        analyzer.analyze_program(&program)?;

        // Execute
        let mut interpreter = codegen::Interpreter::new();
        interpreter
            .execute_program(&program)
            .map_err(|e| OtagError::runtime(e, location::Location::unknown()))?;

        Ok(())
    }

    /// Parse and load a program with all its imports from virtual file system
    fn load_program_with_imports(&self, file_path: &str) -> Result<Program> {
        let mut loaded_files = std::collections::HashSet::new();
        self.load_program_recursive(file_path, &mut loaded_files)
    }

    /// Recursively load program with imports from virtual file system
    fn load_program_recursive(
        &self,
        file_path: &str,
        loaded_files: &mut std::collections::HashSet<String>,
    ) -> Result<Program> {
        // Check for circular imports
        if loaded_files.contains(file_path) {
            // Already loaded, return empty program to avoid duplication
            return Ok(Program {
                statements: Vec::new(),
            });
        }

        // Mark this file as loaded
        loaded_files.insert(file_path.to_string());

        // Read from virtual file system
        let input = self.vfs.get_file(file_path).ok_or_else(|| {
            OtagError::runtime(
                format!("Dosya bulunamadı: {}", file_path),
                location::Location::new(file_path.to_string(), 0, 0),
            )
        })?;

        let program = parser::parse(input, file_path)?;

        // Process imports
        let mut all_statements = Vec::new();
        let base_dir = Path::new(file_path).parent().unwrap_or(Path::new(""));

        for statement in program.statements {
            if let ast::Statement::Import(import_stmt) = &statement {
                // Resolve the import path relative to the current file
                let import_path = base_dir.join(&import_stmt.path);
                let import_path_str = import_path.to_str().ok_or_else(|| {
                    OtagError::runtime(
                        format!("Geçersiz içe aktarma yolu: {}", import_stmt.path),
                        location::Location::unknown(),
                    )
                })?;

                // Recursively load the imported module
                let imported_program =
                    self.load_program_recursive(import_path_str, loaded_files)?;

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

    /// Execute a single in-memory program without imports
    pub fn execute_inline(source: &str) -> Result<()> {
        let program = parser::parse(source, "<inline>")?;

        // Semantic analysis
        let mut analyzer = semantic::SemanticAnalyzer::new();
        analyzer.analyze_program(&program)?;

        // Execute
        let mut interpreter = codegen::Interpreter::new();
        interpreter
            .execute_program(&program)
            .map_err(|e| OtagError::runtime(e, location::Location::unknown()))?;

        Ok(())
    }
}

impl Default for OtagRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_inline_simple_program() {
        let source = r#"
x'ı tamsayı olarak tanımla
x = 5
söyle x
"#;
        let result = OtagRuntime::execute_inline(source);
        assert!(
            result.is_ok(),
            "Failed to execute inline program: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_runtime_with_virtual_files() {
        let mut runtime = OtagRuntime::new();

        runtime.add_source(
            "main.otağ",
            r#"
x'ı tamsayı olarak tanımla
x = 10
söyle x
"#,
        );

        let result = runtime.execute("main.otağ");
        assert!(
            result.is_ok(),
            "Failed to execute program: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_virtual_imports() {
        let mut runtime = OtagRuntime::new();

        // Add a helper module
        runtime.add_source(
            "helper.otağ",
            r#"
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}
"#,
        );

        // Add main program that imports helper
        runtime.add_source(
            "main.otağ",
            r#"
kullan "helper.otağ"

x'ı tamsayı olarak tanımla
x = 5

y'ı tamsayı olarak tanımla
y = 3

sonuç'ı tamsayı olarak tanımla
sonuç = topla(x, y)

söyle sonuç
"#,
        );

        let result = runtime.execute("main.otağ");
        assert!(
            result.is_ok(),
            "Failed to execute program with imports: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_nested_virtual_imports() {
        let mut runtime = OtagRuntime::new();

        // Base utility module
        runtime.add_source(
            "utils.otağ",
            r#"
fonksiyon double(x: tamsayı) -> tamsayı {
    return x + x
}
"#,
        );

        // Math module that imports utils
        runtime.add_source(
            "math.otağ",
            r#"
kullan "utils.otağ"

fonksiyon quadruple(x: tamsayı) -> tamsayı {
    d'ı tamsayı olarak tanımla
    d = double(x)
    return d + d
}
"#,
        );

        // Main program that imports math
        runtime.add_source(
            "main.otağ",
            r#"
kullan "math.otağ"

n'ı tamsayı olarak tanımla
n = 5

sonuç'ı tamsayı olarak tanımla
sonuç = quadruple(n)

söyle sonuç
"#,
        );

        let result = runtime.execute("main.otağ");
        assert!(
            result.is_ok(),
            "Failed to execute nested imports: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_circular_import_protection() {
        let mut runtime = OtagRuntime::new();

        runtime.add_source(
            "a.otağ",
            r#"
kullan "b.otağ"

x'ı tamsayı olarak tanımla
x = 1
söyle x
"#,
        );

        runtime.add_source(
            "b.otağ",
            r#"
kullan "a.otağ"

y'ı tamsayı olarak tanımla
y = 2
söyle y
"#,
        );

        let result = runtime.execute("a.otağ");
        // Should not hang or crash
        assert!(result.is_ok(), "Circular import protection failed");
    }
}
