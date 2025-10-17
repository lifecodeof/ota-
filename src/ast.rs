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
    If(IfStatement),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Break,
    Continue,
    FunctionDefinition(FunctionDefinition),
    Return(Option<Expression>),
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
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub loop_variable: LoopVariable,
    #[allow(dead_code)]
    pub range_start: Box<Expression>,
    #[allow(dead_code)]
    pub range_end: Box<Expression>,
    #[allow(dead_code)]
    pub step: Option<Box<Expression>>,
    #[allow(dead_code)]
    pub body: ControlBlock,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<VariableType>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: VariableType,
}

#[derive(Debug, Clone)]
pub enum Expression {
    VariableRef(String),
    Literal(VariableValue),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}
