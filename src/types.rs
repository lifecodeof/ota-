#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Tamsayi,   // integer
    Metin,     // string
    Ondalikli, // float
    Mantiksal, // boolean
    Array(Box<Type>),
    Struct(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Int(i32),
    String(String),
    Float(f64),
    Bool(bool),
    Array(Vec<VariableValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Syntax,
    Semantic,
    Runtime,
    TypeMismatch,
    UndefinedVariable,
    DivisionByZero,
}

use crate::ast::Expression;

pub fn check_array_elements_types(elements: &Vec<Expression>) -> Result<Type, String> {
    if elements.is_empty() {
        return Err("Empty arrays not supported".to_string());
    }
    let first_type = get_expression_type(&elements[0]);
    for elem in &elements[1..] {
        let elem_type = get_expression_type(elem);
        if elem_type != first_type {
            return Err("All array elements must be of the same type".to_string());
        }
    }
    Ok(Type::Array(Box::new(first_type)))
}

fn get_expression_type(expr: &Expression) -> Type {
    match expr {
        Expression::Literal(val) => match val {
            VariableValue::Int(_) => Type::Tamsayi,
            VariableValue::String(_) => Type::Metin,
            VariableValue::Float(_) => Type::Ondalikli,
            VariableValue::Bool(_) => Type::Mantiksal,
            VariableValue::Array(_) => Type::Array(Box::new(Type::Tamsayi)), // placeholder
        },
        _ => Type::Tamsayi, // placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_type_check() {
        let elements = vec![
            Expression::Literal(VariableValue::Int(1)),
            Expression::Literal(VariableValue::Int(2)),
        ];
        let result = check_array_elements_types(&elements);
        assert_eq!(result, Ok(Type::Array(Box::new(Type::Tamsayi))));
    }
}
