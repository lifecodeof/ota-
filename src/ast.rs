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
    IfStatement(IfStatement),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    BreakStatement,
    ContinueStatement,
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
pub struct ControlBlock {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct LoopVariable {
    pub name: String,
    pub is_auto_generated: bool,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Condition,
    pub then_block: ControlBlock,
    pub else_block: Option<ControlBlock>,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Condition,
    pub body: ControlBlock,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub loop_variable: LoopVariable,
    pub range_start: Box<Expression>,
    pub range_end: Box<Expression>,
    pub step: Option<Box<Expression>>,
    pub body: ControlBlock,
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
    GreaterThan,
    GreaterThanOrEqual,
}
