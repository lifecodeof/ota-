# Parser Contract: Control Flow Statements

**Feature**: Control Flow Statements
**Date**: 2025-10-17
**Type**: Parser Interface Contract

## Overview

This contract defines the interface between the Pest grammar parser and the AST construction for control flow statements in the Otağ language.

## Grammar Rules Added

### Control Flow Keywords
```
eğer    = { "eğer" }
yoksa   = { "yoksa" }
döngü   = { "döngü" }
için    = { "için" }
durdur  = { "durdur" }
devam   = { "devam" }
ise     = { "ise" }
son     = { "son" }
```

### Control Flow Constructs
```
if_statement = {
    eğer ~ condition ~ ise ~ control_block ~
    (yoksa ~ control_block)? ~
    son
}

while_loop = {
    döngü ~ condition ~ ise ~ control_block ~ son
}

for_loop = {
    için ~ identifier ~ in ~ range_spec ~ ise ~ control_block ~ son
}

range_spec = {
    expression ~ dan ~ expression ~ (adım ~ expression)?
}

control_block = {
    statement*
}

break_statement = { durdur }
continue_statement = { devam }
```

## Parser Interface

### Function: parse_if_statement
**Input**: `pest::iterators::Pair<Rule>` (if_statement rule)
**Output**: `Result<ast::IfStatement, ParserError>`
**Contract**:
- Extracts condition expression
- Parses then_block statements
- Parses optional else_block statements
- Returns constructed IfStatement AST node

### Function: parse_while_loop
**Input**: `pest::iterators::Pair<Rule>` (while_loop rule)
**Output**: `Result<ast::WhileLoop, ParserError>`
**Contract**:
- Extracts condition expression
- Parses body statements
- Returns constructed WhileLoop AST node

### Function: parse_for_loop
**Input**: `pest::iterators::Pair<Rule>` (for_loop rule)
**Output**: `Result<ast::ForLoop, ParserError>`
**Contract**:
- Extracts loop variable identifier
- Parses range start, end, and optional step expressions
- Parses body statements
- Returns constructed ForLoop AST node

### Function: parse_control_block
**Input**: `pest::iterators::Pair<Rule>` (control_block rule)
**Output**: `Result<ast::ControlBlock, ParserError>`
**Contract**:
- Parses sequence of statements
- Returns ControlBlock containing statement vector

## Error Handling

### Invalid Condition
**Trigger**: Non-boolean expression in if/while condition
**Response**: ParserError::TypeError with suggestion to use boolean expression

### Missing Loop Variable
**Trigger**: for loop without variable identifier
**Response**: ParserError::SyntaxError with suggestion to provide loop variable

### Malformed Range
**Trigger**: Invalid range specification in for loop
**Response**: ParserError::SyntaxError with suggestion for correct range syntax

## Integration Points

- **Grammar File**: `src/grammar.pest` - Add new rules
- **Parser Module**: `src/parser.rs` - Add parsing functions
- **AST Module**: `src/ast.rs` - Extend Statement enum
- **Error Types**: Extend parser error enum for control flow specific errors
