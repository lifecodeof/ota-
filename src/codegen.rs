use crate::ast::*;
use crate::types::*;
use crate::symbol_table::SymbolTable;
use crate::error_reporting::{OtagError, Result};
use crate::location::Location;
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

    pub fn execute_program(&mut self, program: &Program) -> Result<()> {
        for statement in &program.statements {
            let _ = self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<Option<VariableValue>> {
        match statement {
            Statement::Import(_) => {
                // Import statements are handled at the parsing/loading phase
                // and don't need runtime execution
                Ok(None)
            },
            Statement::VariableDeclaration(decl) => self.execute_variable_declaration(decl),
            Statement::Assignment(assign) => self.execute_assignment(assign),
            Statement::Output(output) => self.execute_output_statement(output),
            Statement::If(if_stmt) => self.execute_if_statement(if_stmt),
            Statement::WhileLoop(while_loop) => self.execute_while_loop(while_loop),
            Statement::ForLoop(for_loop) => self.execute_for_loop(for_loop),
            Statement::Break => self.execute_break(),
            Statement::Continue => self.execute_continue(),
            Statement::FunctionDefinition(func) => {
                self.symbol_table.insert_function(func.clone())
                    .map_err(|e| OtagError::runtime(e, Location::unknown()))?;
                Ok(None)
            },
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let val = self.evaluate_expression(e)?;
                    Ok(Some(val))
                } else {
                    Ok(None)
                }
            },
            Statement::StructDefinition(def) => {
                self.symbol_table.insert_struct(def.clone())
                    .map_err(|e| OtagError::runtime(e, Location::unknown()))?;
                Ok(None)
            },
        }
    }

    fn execute_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<Option<VariableValue>> {
        self.symbol_table.insert(decl.name.clone(), decl.var_type.clone());
        // Initialize with default values
        let default_value = match decl.var_type {
            Type::Tamsayi => VariableValue::Int(0),
            Type::Metin => VariableValue::String(String::new()),
            Type::Ondalikli => VariableValue::Float(0.0),
            Type::Mantiksal => VariableValue::Bool(false),
            _ => return Err(OtagError::runtime(
                format!("Desteklenmeyen tür: {:?}", decl.var_type),
                Location::unknown()
            )),
        };
        self.variables.insert(decl.name.clone(), default_value);
        Ok(None)
    }

    fn execute_assignment(&mut self, assign: &Assignment) -> Result<Option<VariableValue>> {
        let value = self.evaluate_expression(&assign.expression)?;
        self.variables.insert(assign.name.clone(), value);
        Ok(None)
    }

    fn execute_output_statement(&mut self, output: &OutputStatement) -> Result<Option<VariableValue>> {
        let value = self.evaluate_expression(&output.expression)?;
        match value {
            VariableValue::Int(i) => println!("{}", i),
            VariableValue::String(s) => println!("{}", s),
            VariableValue::Float(f) => println!("{}", f),
            VariableValue::Bool(b) => println!("{}", if b { "doğru" } else { "yanlış" }),
            VariableValue::Array(arr) => println!("{:?}", arr),
        }
        Ok(None)
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<VariableValue> {
        match expr {
            Expression::VariableRef(name) => {
                self.variables.get(name).cloned().ok_or_else(|| 
                    OtagError::runtime(format!("Tanımlanmamış değişken: {}", name), Location::unknown())
                )
            },
            Expression::Literal(value) => Ok(value.clone()),
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(left_val, right_val, op)
            },
            Expression::FunctionCall(call) => {
                let func = self.symbol_table.lookup_function(&call.name).ok_or_else(|| 
                    OtagError::runtime(format!("Tanımlanmamış fonksiyon: {}", call.name), Location::unknown())
                )?.clone();
                if call.arguments.len() != func.parameters.len() {
                    return Err(OtagError::runtime(
                        format!("Fonksiyon '{}' {} parametre bekliyor, {} verildi", call.name, func.parameters.len(), call.arguments.len()),
                        Location::unknown()
                    ));
                }
                let mut arg_values = Vec::new();
                for arg in &call.arguments {
                    arg_values.push(self.evaluate_expression(arg)?);
                }
                // Push new scope
                self.symbol_table.push_scope();
                // Set parameters as variables
                for (param, val) in func.parameters.iter().zip(arg_values) {
                    self.symbol_table.insert(param.name.clone(), param.param_type.clone());
                    self.variables.insert(param.name.clone(), val);
                }
                // Execute body
                let mut result = None;
                for stmt in &func.body {
                    let res = self.execute_statement(stmt)?;
                    if let Some(val) = res {
                        result = Some(val);
                        break;
                    }
                }
                // Pop scope
                self.symbol_table.pop_scope();
                // Remove param variables? But since scope popped, and variables are per interpreter, need to remove from variables too.
                // Actually, variables are global, so need to remove the params.
                for param in &func.parameters {
                    self.variables.remove(&param.name);
                }
                result.ok_or_else(|| 
                    OtagError::runtime(format!("Fonksiyon '{}' bir değer döndürmedi", call.name), Location::unknown())
                )
            },
            Expression::ArrayLiteral(array_lit) => {
                let mut values = Vec::new();
                for elem in &array_lit.elements {
                    values.push(self.evaluate_expression(elem)?);
                }
                Ok(VariableValue::Array(values))
            },
            Expression::ArrayAccess(access) => {
                let array_val = self.evaluate_expression(&access.array)?;
                let index_val = self.evaluate_expression(&access.index)?;
                if let VariableValue::Array(arr) = array_val {
                    if let VariableValue::Int(idx) = index_val {
                        if idx >= 0 && (idx as usize) < arr.len() {
                            Ok(arr[idx as usize].clone())
                        } else {
                            Err(OtagError::runtime(
                                format!("Dizi indeksi sınırların dışında: {} (dizi uzunluğu: {})", idx, arr.len()),
                                Location::unknown()
                            ))
                        }
                    } else {
                        Err(OtagError::runtime(
                            "Dizi indeksi tamsayı olmalıdır".to_string(),
                            Location::unknown()
                        ))
                    }
                } else {
                    Err(OtagError::runtime(
                        "Dizi değil, dizi erişimi yapılamaz".to_string(),
                        Location::unknown()
                    ))
                }
            },
            Expression::StructLiteral(_) => todo!(),
            Expression::StructAccess(_) => todo!(),
        }
    }

    fn evaluate_binary_op(&self, left: VariableValue, right: VariableValue, op: &BinaryOperator) -> Result<VariableValue> {
        match op {
            BinaryOperator::Add => self.add_values(left, right),
            BinaryOperator::GreaterThan => self.compare_greater(left, right),
            BinaryOperator::GreaterThanOrEqual => self.compare_greater_equal(left, right),
            BinaryOperator::LessThan => self.compare_less(left, right),
            BinaryOperator::LessThanOrEqual => self.compare_less_equal(left, right),
        }
    }

    fn add_values(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Int(l + r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Float(l + r)),
            (VariableValue::String(l), VariableValue::String(r)) => Ok(VariableValue::String(l + &r)),
            (left_val, right_val) => Err(OtagError::runtime(
                format!("Tür uyumsuzluğu: {:?} ve {:?} türündeki değerler toplanamaz. Toplama işlemi sadece aynı sayısal türler veya metinler arasında desteklenir.", left_val, right_val),
                Location::unknown()
            )),
        }
    }

    fn compare_greater(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l > r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l > r)),
            (left_val, right_val) => Err(OtagError::runtime(
                format!("Tür uyumsuzluğu: {:?} ve {:?} türündeki değerler > operatörü ile karşılaştırılamaz. Karşılaştırma sadece aynı sayısal türler arasında desteklenir.", left_val, right_val),
                Location::unknown()
            )),
        }
    }

    fn compare_greater_equal(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l >= r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l >= r)),
            (left_val, right_val) => Err(OtagError::runtime(
                format!("Tür uyumsuzluğu: {:?} ve {:?} türündeki değerler >= operatörü ile karşılaştırılamaz. Karşılaştırma sadece aynı sayısal türler arasında desteklenir.", left_val, right_val),
                Location::unknown()
            )),
        }
    }

    fn compare_less(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l < r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l < r)),
            (left_val, right_val) => Err(OtagError::runtime(
                format!("Tür uyumsuzluğu: {:?} ve {:?} türündeki değerler < operatörü ile karşılaştırılamaz. Karşılaştırma sadece aynı sayısal türler arasında desteklenir.", left_val, right_val),
                Location::unknown()
            )),
        }
    }

    fn compare_less_equal(&self, left: VariableValue, right: VariableValue) -> Result<VariableValue> {
        match (left, right) {
            (VariableValue::Int(l), VariableValue::Int(r)) => Ok(VariableValue::Bool(l <= r)),
            (VariableValue::Float(l), VariableValue::Float(r)) => Ok(VariableValue::Bool(l <= r)),
            (left_val, right_val) => Err(OtagError::runtime(
                format!("Tür uyumsuzluğu: {:?} ve {:?} türündeki değerler <= operatörü ile karşılaştırılamaz. Karşılaştırma sadece aynı sayısal türler arasında desteklenir.", left_val, right_val),
                Location::unknown()
            )),
        }
    }

    fn execute_control_block(&mut self, block: &ControlBlock) -> Result<()> {
        for statement in &block.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_if_statement(&mut self, if_stmt: &IfStatement) -> Result<Option<VariableValue>> {
        let condition_value = self.evaluate_expression(&if_stmt.condition.expression)?;
        if let VariableValue::Bool(cond) = condition_value {
            if cond {
                self.execute_control_block(&if_stmt.then_block)?;
            } else if let Some(else_block) = &if_stmt.else_block {
                self.execute_control_block(else_block)?;
            }
            Ok(None)
        } else {
            Err(OtagError::runtime(
                "Eğer koşulu mantıksal (doğru/yanlış) bir değer döndürmelidir".to_string(),
                Location::unknown()
            ))
        }
    }

    fn execute_while_loop(&mut self, while_loop: &WhileLoop) -> Result<Option<VariableValue>> {
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10000; // Prevent infinite loops

        loop {
            if iterations >= MAX_ITERATIONS {
                return Err(OtagError::runtime(
                    "Döngü maksimum iterasyon sayısını aştı (10000). Sonsuz döngü olabilir.".to_string(),
                    Location::unknown()
                ));
            }

            let condition_value = self.evaluate_expression(&while_loop.condition.expression)?;
            if let VariableValue::Bool(cond) = condition_value {
                if !cond {
                    break;
                }
                self.execute_control_block(&while_loop.body)?;
                iterations += 1;
            } else {
                return Err(OtagError::runtime(
                    "Döngü koşulu mantıksal (doğru/yanlış) bir değer döndürmelidir".to_string(),
                    Location::unknown()
                ));
            }
        }
        Ok(None)
    }

    fn execute_for_loop(&mut self, _for_loop: &ForLoop) -> Result<Option<VariableValue>> {
        Err(OtagError::runtime(
            "For döngüsü henüz uygulanmadı".to_string(),
            Location::unknown()
        ))
    }

    fn execute_break(&mut self) -> Result<Option<VariableValue>> {
        Err(OtagError::runtime(
            "Break ifadesi henüz uygulanmadı".to_string(),
            Location::unknown()
        ))
    }

    fn execute_continue(&mut self) -> Result<Option<VariableValue>> {
        Err(OtagError::runtime(
            "Continue ifadesi henüz uygulanmadı".to_string(),
            Location::unknown()
        ))
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
            var_type: Type::Tamsayi,
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
            var_type: Type::Tamsayi,
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
