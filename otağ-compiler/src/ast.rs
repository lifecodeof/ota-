use crate::types::{VariableType, VariableValue};

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    Output(OutputStatement),
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: VariableType,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub struct OutputStatement {
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub enum Expression {
    VariableRef(String),
    Literal(VariableValue),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
}
