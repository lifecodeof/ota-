use crate::ast::*;
use crate::types::*;
use crate::symbol_table::SymbolTable;
use std::collections::HashMap;

pub struct Interpreter {
    pub symbol_table: SymbolTable,
    pub variables: HashMap<String, VariableValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            symbol_table: SymbolTable::new(),
            variables: HashMap::new(),
        }
    }

    pub fn execute_program(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::VariableDeclaration(decl) => self.execute_variable_declaration(decl),
            Statement::Assignment(assign) => self.execute_assignment(assign),
            Statement::OutputStatement(output) => self.execute_output_statement(output),
        }
    }

    fn execute_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<(), String> {
        self.symbol_table.insert(decl.name.clone(), decl.var_type.clone());
        // Initialize with default values
        let default_value = match decl.var_type {
            VariableType::Tamsayi => VariableValue::Int(0),
            VariableType::Metin => VariableValue::String(String::new()),
            VariableType::Ondalikli => VariableValue::Float(0.0),
            VariableType::Mantiksal => VariableValue::Bool(false),
        };
        self.variables.insert(decl.name.clone(), default_value);
        Ok(())
    }

    fn execute_assignment(&mut self, assign: &Assignment) -> Result<(), String> {
        let value = self.evaluate_expression(&assign.expression)?;
        self.variables.insert(assign.name.clone(), value);
        Ok(())
    }

    fn execute_output_statement(&mut self, output: &OutputStatement) -> Result<(), String> {
        let value = self.evaluate_expression(&output.expression)?;
        match value {
            VariableValue::Int(i) => println!("{}", i),
            VariableValue::String(s) => println!("{}", s),
            VariableValue::Float(f) => println!("{}", f),
            VariableValue::Bool(b) => println!("{}", if b { "doğru" } else { "yanlış" }),
        }
        Ok(())
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<VariableValue, String> {
        match expr {
            Expression::VariableRef(name) => {
                self.variables.get(name).cloned().ok_or_else(|| format!("Undefined variable: {}", name))
            },
            Expression::Literal(value) => Ok(value.clone()),
            _ => Err("Complex expressions not yet supported".to_string()),
        }
    }
}
