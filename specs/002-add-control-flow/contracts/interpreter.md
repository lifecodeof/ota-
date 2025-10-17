# Interpreter Contract: Control Flow Statements

**Feature**: Control Flow Statements
**Date**: 2025-10-17
**Type**: Execution Interface Contract

## Overview

This contract defines the execution interface for control flow statements in the Otağ interpreter.

## Execution Interface

### Function: execute_if_statement
**Input**: `&ast::IfStatement`, `&mut SymbolTable`, `&mut VariableStore`
**Output**: `Result<(), InterpreterError>`
**Contract**:
- Evaluates condition expression to boolean
- If true, executes then_block statements
- If false and else_block exists, executes else_block statements
- Returns execution result

### Function: execute_while_loop
**Input**: `&ast::WhileLoop`, `&mut SymbolTable`, `&mut VariableStore`
**Output**: `Result<(), InterpreterError>`
**Contract**:
- Evaluates condition before each iteration
- If condition is true, executes body statements
- Repeats until condition becomes false
- Implements iteration limit (10,000) for infinite loop prevention
- Handles break/continue statements within loop

### Function: execute_for_loop
**Input**: `&ast::ForLoop`, `&mut SymbolTable`, `&mut VariableStore`
**Output**: `Result<(), InterpreterError>`
**Contract**:
- Evaluates range start, end, and step expressions
- Creates loop variable in symbol table with proper scoping
- Iterates from start to end (exclusive) with specified step
- Assigns current value to loop variable each iteration
- Executes body statements for each iteration
- Cleans up loop variable after completion
- Handles break/continue statements within loop

### Function: execute_break
**Input**: Current execution context
**Output**: `Result<ControlFlow, InterpreterError>`
**Contract**:
- Signals immediate exit from innermost loop
- Returns ControlFlow::Break to execution engine
- Only valid within loop contexts

### Function: execute_continue
**Input**: Current execution context
**Output**: `Result<ControlFlow, InterpreterError>`
**Contract**:
- Signals immediate jump to next loop iteration
- Returns ControlFlow::Continue to execution engine
- Only valid within loop contexts

## Control Flow Types

### ControlFlow Enum
```
enum ControlFlow {
    Normal,      // Continue normal execution
    Break,       // Exit innermost loop
    Continue,    // Skip to next iteration
    Return,      // Future: return from function
}
```

## Error Handling

### Type Error in Condition
**Trigger**: Condition expression doesn't evaluate to boolean
**Response**: InterpreterError::TypeError with context about expected boolean

### Undefined Loop Variable
**Trigger**: Loop variable referenced outside loop scope
**Response**: InterpreterError::ScopeError with variable name and scope information

### Break/Continue Outside Loop
**Trigger**: break or continue statement outside loop context
**Response**: InterpreterError::SyntaxError with suggestion to use within loops

### Infinite Loop Detected
**Trigger**: Loop exceeds 10,000 iterations
**Response**: InterpreterError::InfiniteLoopError with loop location and iteration count

## State Management

### Symbol Table Updates
- Loop variables added to symbol table at loop start
- Loop variables removed from symbol table at loop end
- Nested loops maintain separate variable scopes

### Execution Context
- Track current loop depth for break/continue validation
- Maintain call stack for error reporting
- Preserve variable state across control flow operations

## Performance Characteristics

- **Condition Evaluation**: O(expression complexity) per evaluation
- **Loop Overhead**: Minimal additional cost per iteration
- **Memory Usage**: O(loop depth) for scope management
- **Infinite Loop Protection**: Constant-time iteration counting
