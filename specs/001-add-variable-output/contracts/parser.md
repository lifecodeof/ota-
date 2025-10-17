# Parser Contract

**Interface**: `Parser`

**Purpose**: Parse token stream into Abstract Syntax Tree.

**Input**: `&[Token]`

**Output**: `Result<Program, ParseError>`

**AST Types** (subset):
- Program: Vec<Statement>
- Statement: VariableDeclaration | Assignment | OutputStatement
- VariableDeclaration: name, type
- Assignment: name, expression
- OutputStatement: expression
- Expression: VariableRef | Literal

**Error Handling**: Syntax errors, unexpected tokens, type mismatches.

**Constraints**: Must validate variable names and types per data model.
