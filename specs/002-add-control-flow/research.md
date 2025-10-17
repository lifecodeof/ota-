# Research: Control Flow Statements

**Feature**: Control Flow Statements
**Date**: 2025-10-17
**Status**: Complete

## Research Tasks

### Task 1: AST Design for Control Flow Constructs
**Question**: How to extend the existing AST to support if/else, while, and for statements?

**Findings**:
- Current AST has: Program, Statement (VariableDeclaration, Assignment, Output), Expression (VariableRef, Literal, BinaryOp)
- Need to add: IfStatement, WhileLoop, ForLoop, BreakStatement, ContinueStatement
- Pattern: Follow existing structure with Box<Expression> for conditions and Vec<Statement> for bodies

**Decision**: Extend Statement enum with control flow variants
**Rationale**: Maintains consistency with existing AST design patterns
**Alternatives Considered**: Separate control flow AST - rejected for complexity

### Task 2: Pest Grammar Patterns for Control Flow
**Question**: How to write Pest grammar rules for Turkish control flow keywords?

**Findings**:
- Existing grammar supports Turkish identifiers and expressions
- Need rules for: if/else blocks, while loops, for loops, break/continue
- Pattern: Use existing expression parsing and extend statement rules

**Decision**: Add control flow rules to grammar.pest following existing patterns
**Rationale**: Leverages existing Turkish keyword support and expression parsing
**Alternatives Considered**: Custom parser logic - rejected for grammar maintainability

### Task 3: Interpreter Execution Logic for Control Flow
**Question**: How to implement control flow execution in the interpreter?

**Findings**:
- Current interpreter has execute_statement with pattern matching
- Need to add cases for: if evaluation, loop iteration, break/continue handling
- Pattern: Use Rust control flow (if/while/for) with recursive statement execution

**Decision**: Extend Interpreter::execute_statement with control flow cases
**Rationale**: Maintains existing execution model and error handling patterns
**Alternatives Considered**: Separate control flow interpreter - rejected for unnecessary complexity

### Task 4: Loop Variable Scoping and Management
**Question**: How to handle loop variables in for loops and scoping rules?

**Findings**:
- For loops need iteration variables (e.g., "i" in "için i in range")
- Variables should be scoped to the loop body
- Pattern: Extend symbol table with loop variable management

**Decision**: Add loop variable handling to symbol table with proper scoping
**Rationale**: Ensures clean variable lifecycle and prevents conflicts
**Alternatives Considered**: Global loop variables - rejected for safety concerns

### Task 5: Infinite Loop Prevention Implementation
**Question**: How to implement automatic infinite loop detection?

**Findings**:
- Need iteration counting in loops
- Reasonable threshold: 10,000 iterations (configurable)
- Pattern: Add iteration counter to loop execution with early termination

**Decision**: Implement iteration limit checking in loop execution
**Rationale**: Provides safety for educational use while allowing intentional long-running loops
**Alternatives Considered**: No protection - rejected for user safety

### Task 6: Error Message Patterns for Control Flow
**Question**: How to implement descriptive error messages with suggestions?

**Findings**:
- Need specific error types for control flow syntax errors
- Pattern: Include expected vs actual tokens in error messages
- Examples: "Expected 'ise' after condition, found 'is'"

**Decision**: Extend error handling with control flow specific error types
**Rationale**: Provides helpful debugging information for users
**Alternatives Considered**: Generic errors - rejected for poor user experience

## Implementation Approach

**Architecture**: Extend existing modular design (lexer → parser → AST → interpreter)

**Key Components**:
1. **Lexer**: Add Turkish control flow keywords
2. **Parser**: Add grammar rules for control flow syntax
3. **AST**: Add control flow node types
4. **Interpreter**: Add execution logic for control flow
5. **Symbol Table**: Add loop variable scoping

**Risks Identified**:
- Complex nested control flow parsing
- Loop variable scoping edge cases
- Performance impact of iteration counting

**Mitigations**:
- Start with simple cases, add complexity incrementally
- Comprehensive unit testing for scoping
- Profile performance impact before optimization
