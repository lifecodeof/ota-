use crate::ast::*;
use crate::symbol_table::SymbolTable;

pub struct CodeGen {
    pub symbol_table: SymbolTable,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn generate_program(&mut self, program: &Program) -> Result<(), String> {
        println!("; Generated code for Otağ program");
        for statement in &program.statements {
            self.generate_statement(statement)?;
        }
        println!("; End of program");
        Ok(())
    }

    fn generate_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::VariableDeclaration(decl) => self.generate_variable_declaration(decl),
            Statement::Assignment(assign) => self.generate_assignment(assign),
            Statement::OutputStatement(output) => self.generate_output_statement(output),
        }
    }

    fn generate_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<(), String> {
        self.symbol_table.insert(decl.name.clone(), decl.var_type.clone());
        println!("; Declare {} as {:?}", decl.name, decl.var_type);
        Ok(())
    }

    fn generate_assignment(&mut self, assign: &Assignment) -> Result<(), String> {
        println!("; Assign to {}", assign.name);
        Ok(())
    }

    fn generate_output_statement(&mut self, _output: &OutputStatement) -> Result<(), String> {
        println!("; Output statement");
        Ok(())
    }
}
