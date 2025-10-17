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
            Statement::IfStatement(if_stmt) => self.execute_if_statement(if_stmt),
            Statement::WhileLoop(while_loop) => self.execute_while_loop(while_loop),
            Statement::ForLoop(for_loop) => self.execute_for_loop(for_loop),
            Statement::BreakStatement => self.execute_break(),
            Statement::ContinueStatement => self.execute_continue(),
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
            BinaryOperator::GreaterThan => self.compare_greater(left, right),
            BinaryOperator::GreaterThanOrEqual => self.compare_greater_equal(left, right),
            BinaryOperator::LessThan => self.compare_less(left, right),
            BinaryOperator::LessThanOrEqual => self.compare_less_equal(left, right),
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

    fn compare_greater(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue, String> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l > r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l > r)),
            (left_val, right_val) => Err(format!("Cannot compare values of types {:?} and {:?} with >. Comparison is only supported between matching numeric types.", left_val, right_val)),
        }
    }

    fn compare_greater_equal(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue, String> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l >= r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l >= r)),
            (left_val, right_val) => Err(format!("Cannot compare values of types {:?} and {:?} with >=. Comparison is only supported between matching numeric types.", left_val, right_val)),
        }
    }

    fn compare_less(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue, String> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l < r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l < r)),
            (left_val, right_val) => Err(format!("Cannot compare values of types {:?} and {:?} with <. Comparison is only supported between matching numeric types.", left_val, right_val)),
        }
    }

    fn compare_less_equal(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue, String> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l <= r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l <= r)),
            (left_val, right_val) => Err(format!("Cannot compare values of types {:?} and {:?} with <=. Comparison is only supported between matching numeric types.", left_val, right_val)),
        }
    }

    fn execute_control_block(&mut self, block: &ControlBlock) -> Result<(), String> {
        for statement in &block.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_if_statement(&mut self, if_stmt: &IfStatement) -> Result<(), String> {
        let condition_value = self.evaluate_expression(&if_stmt.condition.expression)?;
        if let VariableValue::Bool(cond) = condition_value {
            if cond {
                self.execute_control_block(&if_stmt.then_block)?;
            } else if let Some(else_block) = &if_stmt.else_block {
                self.execute_control_block(else_block)?;
            }
            Ok(())
        } else {
            Err("If condition must evaluate to a boolean".to_string())
        }
    }

    fn execute_while_loop(&mut self, while_loop: &WhileLoop) -> Result<(), String> {
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10000; // Prevent infinite loops

        loop {
            if iterations >= MAX_ITERATIONS {
                return Err("While loop exceeded maximum iterations (10000). Possible infinite loop.".to_string());
            }

            let condition_value = self.evaluate_expression(&while_loop.condition.expression)?;
            if let VariableValue::Bool(cond) = condition_value {
                if !cond {
                    break;
                }
                self.execute_control_block(&while_loop.body)?;
                iterations += 1;
            } else {
                return Err("While loop condition must evaluate to a boolean".to_string());
            }
        }
        Ok(())
    }

    fn execute_for_loop(&mut self, for_loop: &ForLoop) -> Result<(), String> {
        todo!("Implement for loop execution")
    }

    fn execute_break(&mut self) -> Result<(), String> {
        todo!("Implement break statement")
    }

    fn execute_continue(&mut self) -> Result<(), String> {
        todo!("Implement continue statement")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_if_statement() {
        let mut interpreter = Interpreter::new();
        // Declare x = 10
        let decl = VariableDeclaration {
            name: "x".to_string(),
            var_type: VariableType::Tamsayi,
        };
        interpreter.execute_variable_declaration(&decl).unwrap();
        interpreter.execute_assignment(&Assignment {
            name: "x".to_string(),
            expression: Expression::Literal(VariableValue::Int(10)),
        }).unwrap();

        // If x > 5 then output "Büyük"
        let if_stmt = IfStatement {
            condition: Condition {
                expression: Box::new(Expression::BinaryOp(
                    Box::new(Expression::VariableRef("x".to_string())),
                    BinaryOperator::GreaterThan,
                    Box::new(Expression::Literal(VariableValue::Int(5))),
                )),
            },
            then_block: ControlBlock {
                statements: vec![Statement::Output(OutputStatement {
                    expression: Expression::Literal(VariableValue::String("Büyük".to_string())),
                })],
            },
            else_block: None,
        };

        interpreter.execute_if_statement(&if_stmt).unwrap();
        // Should have printed "Büyük"
    }

    #[test]
    fn test_execute_while_loop() {
        let mut interpreter = Interpreter::new();
        // Declare counter = 0
        let decl = VariableDeclaration {
            name: "counter".to_string(),
            var_type: VariableType::Tamsayi,
        };
        interpreter.execute_variable_declaration(&decl).unwrap();
        interpreter.execute_assignment(&Assignment {
            name: "counter".to_string(),
            expression: Expression::Literal(VariableValue::Int(0)),
        }).unwrap();

        // While counter < 3, increment counter and output its value
        let while_loop = WhileLoop {
            condition: Condition {
                expression: Box::new(Expression::BinaryOp(
                    Box::new(Expression::VariableRef("counter".to_string())),
                    BinaryOperator::LessThan,
                    Box::new(Expression::Literal(VariableValue::Int(3))),
                )),
            },
            body: ControlBlock {
                statements: vec![
                    Statement::Output(OutputStatement {
                        expression: Expression::VariableRef("counter".to_string()),
                    }),
                    Statement::Assignment(Assignment {
                        name: "counter".to_string(),
                        expression: Expression::BinaryOp(
                            Box::new(Expression::VariableRef("counter".to_string())),
                            BinaryOperator::Add,
                            Box::new(Expression::Literal(VariableValue::Int(1))),
                        ),
                    }),
                ],
            },
        };

        interpreter.execute_while_loop(&while_loop).unwrap();
        // Should have printed 0, 1, 2
    }
}
