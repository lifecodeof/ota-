# Grammar Contract: Control Flow Statements

**Feature**: Control Flow Statements
**Date**: 2025-10-17
**Type**: Grammar Rule Specification

## Overview

This contract specifies the Pest grammar rules for parsing control flow statements in the Otağ language.

## Grammar Rules

### Keywords (Turkish)
```
eğer_keyword    = { "eğer" }
yoksa_keyword   = { "yoksa" }
döngü_keyword   = { "döngü" }
için_keyword    = { "için" }
durdur_keyword  = { "durdur" }
devam_keyword   = { "devam" }
ise_keyword     = { "ise" }
son_keyword     = { "son" }
dan_keyword     = { "dan" }
adım_keyword    = { "adım" }
in_keyword      = { "in" }
```

### Control Flow Statements
```
if_statement = {
    eğer_keyword ~ WHITESPACE ~ condition ~ WHITESPACE ~
    ise_keyword ~ WHITESPACE ~ control_block ~
    (WHITESPACE ~ yoksa_keyword ~ WHITESPACE ~ control_block)? ~
    WHITESPACE ~ son_keyword
}

while_statement = {
    döngü_keyword ~ WHITESPACE ~ condition ~ WHITESPACE ~
    ise_keyword ~ WHITESPACE ~ control_block ~
    WHITESPACE ~ son_keyword
}

for_statement = {
    için_keyword ~ WHITESPACE ~ identifier ~ WHITESPACE ~
    in_keyword ~ WHITESPACE ~ range_spec ~ WHITESPACE ~
    ise_keyword ~ WHITESPACE ~ control_block ~
    WHITESPACE ~ son_keyword
}

break_statement = {
    durdur_keyword
}

continue_statement = {
    devam_keyword
}
```

### Supporting Rules
```
condition = { expression }

control_block = {
    NEWLINE*
    statement*
    NEWLINE*
}

range_spec = {
    expression ~ WHITESPACE ~ dan_keyword ~ WHITESPACE ~
    expression ~
    (WHITESPACE ~ adım_keyword ~ WHITESPACE ~ expression)?
}

statement = {
    if_statement |
    while_statement |
    for_statement |
    break_statement |
    continue_statement |
    variable_declaration |
    assignment |
    output_statement
}
```

## Grammar Integration

### File Location
- **File**: `src/grammar.pest`
- **Integration Point**: Extend existing `statement` rule

### Precedence Rules
1. Keywords must be properly tokenized
2. Whitespace handling between tokens
3. Nested expression parsing in conditions
4. Block structure with proper termination

## Validation Rules

### Syntactic Correctness
- All keywords must be present in correct order
- Whitespace requirements must be satisfied
- Block termination with `son` keyword
- Proper nesting of expressions

### Semantic Constraints
- Conditions must be boolean expressions
- Range specifications must be numeric
- Loop variables must be valid identifiers
- Control flow statements only valid in appropriate contexts

## Error Recovery

### Common Syntax Errors
- Missing `ise` after condition
- Missing `son` at block end
- Invalid range specifications
- Malformed control block structure

### Error Messages
- Include expected token in error
- Suggest common corrections
- Provide context about error location
