use crate::types::{VariableType, VariableValue};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    OutputStatement(OutputStatement),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: VariableType,
}

#[derive(Debug)]
pub struct Assignment {
    pub name: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct OutputStatement {
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
    VariableRef(String),
    Literal(VariableValue),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
}
