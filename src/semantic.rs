use crate::ast::*;
use crate::error_reporting::*;
use crate::location::*;
use crate::symbol_table::SymbolTable;
use crate::types::*;

// Semantic analysis phase
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn analyze_program(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }
        Ok(())
    }

    fn analyze_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::VariableDeclaration(decl) => {
                // Check if variable already declared
                if self.symbol_table.lookup(&decl.name).is_some() {
                    return Err(OtagError::semantic(
                        format!("Değişken '{}' zaten tanımlanmış", decl.name),
                        Location::unknown(),
                    ));
                }
                self.symbol_table.insert(decl.name.clone(), decl.var_type.clone());
            }
            Statement::Assignment(assign) => {
                // Check if variable is declared
                if self.symbol_table.lookup(&assign.name).is_none() {
                    return Err(OtagError::semantic(
                        format!("Tanımlanmamış değişken: {}", assign.name),
                        Location::unknown(),
                    ));
                }
                // TODO: Type check assignment
            }
            Statement::FunctionDefinition(func) => {
                self.symbol_table.insert_function(func.clone()).map_err(|e| OtagError::semantic(e, Location::unknown()))?;
            }
            Statement::StructDefinition(def) => {
                self.symbol_table.insert_struct(def.clone()).map_err(|e| OtagError::semantic(e, Location::unknown()))?;
            }
            _ => {} // Other statements don't need semantic checks yet
        }
        Ok(())
    }
}
