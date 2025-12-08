use crate::types::{Type, VariableValue};

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Import(ImportStatement),
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    Output(OutputStatement),
    If(IfStatement),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Break,
    Continue,
    #[allow(dead_code)]
    FunctionDefinition(FunctionDefinition),
    #[allow(dead_code)]
    Return(Option<Expression>),
    StructDefinition(StructDefinition),
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: Type,
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
#[allow(dead_code)]
pub struct FunctionDefinition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
    pub element_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct StructLiteral {
    pub struct_name: String,
    pub fields: Vec<FieldAssignment>,
}

#[derive(Debug, Clone)]
pub struct FieldAssignment {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct ArrayAccess {
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct StructAccess {
    pub struct_expr: Box<Expression>,
    pub field_name: String,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: Type,
}

#[derive(Debug, Clone)]
pub enum Expression {
    VariableRef(String),
    Literal(VariableValue),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    #[allow(dead_code)]
    FunctionCall(FunctionCall),
    ArrayLiteral(ArrayLiteral),
    StructLiteral(StructLiteral),
    ArrayAccess(ArrayAccess),
    StructAccess(StructAccess),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
