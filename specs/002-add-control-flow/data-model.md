# Data Model: Control Flow Statements

**Feature**: Control Flow Statements
**Date**: 2025-10-17
**Related**: [spec.md](spec.md), [research.md](research.md)

## Overview

The control flow feature extends the Otağ language AST with conditional and iterative constructs. This document describes the data structures and relationships for implementing if/else statements, while loops, for loops, and break/continue statements.

## AST Extensions

### Core Entities

#### ControlBlock
**Purpose**: Represents executable code blocks in control flow constructs
**Fields**:
- `statements: Vec<Statement>` - Ordered list of statements to execute
**Relationships**:
- Contained by: IfStatement, WhileLoop, ForLoop
- Contains: Statement (recursive relationship)
**Validation Rules**:
- Must contain at least one statement (empty blocks allowed but must be explicit)
- Statements execute in order

#### Condition
**Purpose**: Boolean expression that controls execution flow
**Fields**:
- `expression: Box<Expression>` - Boolean expression to evaluate
**Relationships**:
- Used by: IfStatement, WhileLoop
- References: Expression (VariableRef, Literal, BinaryOp)
**Validation Rules**:
- Must evaluate to boolean type at runtime
- Can contain variables, literals, and binary operations

#### LoopVariable
**Purpose**: Represents iteration variables in for loops
**Fields**:
- `name: String` - Variable identifier
- `is_auto_generated: bool` - Whether variable was implicitly created
**Relationships**:
- Owned by: ForLoop
- Scoped to: ControlBlock of the loop
**Validation Rules**:
- Must be unique within loop scope
- Cannot shadow existing variables in outer scope

### Control Flow Node Types

#### IfStatement
**Purpose**: Conditional execution with optional else branch
**Fields**:
- `condition: Condition` - Expression that determines execution
- `then_block: ControlBlock` - Statements to execute if condition is true
- `else_block: Option<ControlBlock>` - Statements to execute if condition is false
**Relationships**:
- condition → Condition
- then_block, else_block → ControlBlock
**Validation Rules**:
- condition must be present
- then_block must be present
- else_block is optional

#### WhileLoop
**Purpose**: Repeated execution while condition remains true
**Fields**:
- `condition: Condition` - Expression checked before each iteration
- `body: ControlBlock` - Statements to execute in each iteration
**Relationships**:
- condition → Condition
- body → ControlBlock
**Validation Rules**:
- condition must be present
- body must be present
- Infinite loop detection applies

#### ForLoop
**Purpose**: Iteration over ranges with automatic variable management
**Fields**:
- `loop_variable: LoopVariable` - Iteration variable
- `range_start: Box<Expression>` - Starting value of range
- `range_end: Box<Expression>` - Ending value of range (exclusive)
- `step: Option<Box<Expression>>` - Increment value (default: 1)
- `body: ControlBlock` - Statements to execute for each iteration
**Relationships**:
- loop_variable → LoopVariable
- range_start, range_end, step → Expression
- body → ControlBlock
**Validation Rules**:
- loop_variable must be present
- range_start and range_end must be numeric expressions
- step must be numeric if provided
- Loop variable is automatically assigned on each iteration

#### BreakStatement
**Purpose**: Exit from innermost loop prematurely
**Fields**: None (context-dependent)
**Relationships**:
- Must be contained within a loop (WhileLoop or ForLoop)
**Validation Rules**:
- Only valid within loop contexts
- Causes immediate exit from innermost loop

#### ContinueStatement
**Purpose**: Skip to next iteration of innermost loop
**Fields**: None (context-dependent)
**Relationships**:
- Must be contained within a loop (WhileLoop or ForLoop)
**Validation Rules**:
- Only valid within loop contexts
- Causes immediate jump to next iteration

## State Transitions

### Program Execution Flow
1. **Sequential**: Statements execute in order by default
2. **Conditional**: IfStatement branches based on condition evaluation
3. **Iterative**: WhileLoop/ForLoop repeat execution based on conditions
4. **Transfer**: BreakStatement/ContinueStatement alter normal flow within loops

### Variable Lifecycle
- **Declaration**: Variables created at statement execution time
- **Scope**: Loop variables exist only within loop body
- **Cleanup**: Loop variables automatically removed after loop completion

## Data Integrity Rules

- **Type Safety**: All expressions must maintain type consistency
- **Scope Isolation**: Loop variables cannot escape loop scope
- **Execution Order**: Conditions evaluated before body execution
- **Termination Guarantee**: Infinite loops automatically prevented

## Relationships Summary

```
Program
├── Statement (extended with control flow)
    ├── IfStatement
    │   ├── condition: Condition
    │   ├── then_block: ControlBlock
    │   └── else_block: Option<ControlBlock>
    ├── WhileLoop
    │   ├── condition: Condition
    │   └── body: ControlBlock
    ├── ForLoop
    │   ├── loop_variable: LoopVariable
    │   ├── range_start: Expression
    │   ├── range_end: Expression
    │   ├── step: Option<Expression>
    │   └── body: ControlBlock
    ├── BreakStatement
    ├── ContinueStatement
    └── (existing: VariableDeclaration, Assignment, Output)
```
