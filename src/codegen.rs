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
            Statement::Output(output) => self.execute_output_statement(output),
            Statement::IfStatement(_) => todo!("Implement if statement execution"),
            Statement::WhileLoop(_) => todo!("Implement while loop execution"),
            Statement::ForLoop(_) => todo!("Implement for loop execution"),
            Statement::BreakStatement => todo!("Implement break statement"),
            Statement::ContinueStatement => todo!("Implement continue statement"),
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
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(left_val, right_val, op)
            },
        }
    }

    fn evaluate_binary_op(&self, left: VariableValue, right: VariableValue, op: &BinaryOperator) -> Result<VariableValue, String> {
        match op {
            BinaryOperator::Add => self.add_values(left, right),
        }
    }

    fn add_values(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue, String> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Int(l + r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Float(l + r)),
            (VariableValue::String(l), VariableValue::String(r)) => Ok(VariableValue::String(l + &r)),
            (left_val, right_val) => Err(format!("Cannot add values of types {:?} and {:?}. Addition is only supported between matching numeric types or strings.", left_val, right_val)),
        }
    }
}
