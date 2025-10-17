# Quickstart Guide: Add Data Structures

**Feature**: Add Data Structures  
**Date**: 2025-10-17  

## Overview

This guide provides step-by-step instructions to implement arrays and structs in the Otağ compiler. The implementation extends the existing lexer, parser, AST, type system, and codegen components.

## Prerequisites

- Rust 1.75+ installed
- LLVM development libraries
- Existing Otağ compiler codebase
- Basic understanding of compiler construction

## Implementation Steps

### 1. Extend the Lexer (lexer.rs)

Add new tokens for array and struct syntax:

```rust
// Add to Token enum
ArrayStart,      // [
ArrayEnd,        // ]
StructStart,     // {
StructEnd,       // }
Dot,             // .
Colon,           // :
```

Update the lexer to recognize `[` `]` `{` `}` `.` `:` characters.

### 2. Extend the AST (ast.rs)

Add new AST node types as defined in `contracts/ast.md`:

- `ArrayLiteral`
- `StructLiteral`
- `ArrayAccess`
- `StructAccess`
- `StructDefinition`

Extend `Type` enum to include `Array(Box<Type>)` and `Struct(String)`.

Extend `Expression` and `Statement` enums to include the new nodes.

### 3. Extend the Parser (parser.rs)

Add parsing functions for:

- `parse_array_literal()` - Parse `[expr, expr, ...]`
- `parse_struct_definition()` - Parse `Name { field: type, ... }`
- `parse_struct_literal()` - Parse `Name { field: expr, ... }`
- `parse_array_access()` - Parse `expr[index]`
- `parse_struct_access()` - Parse `expr.field`

Update expression parsing to handle the new syntax.

### 4. Extend the Type System (types.rs)

Implement type checking for:

- Array literals: Ensure all elements same type
- Struct literals: Match field names and types to definition
- Array/struct access: Validate types and bounds
- Struct definitions: Register in symbol table

### 5. Extend Code Generation (codegen.rs)

Implement LLVM code generation for:

- Arrays: Stack allocation, element access via getelementptr
- Structs: LLVM struct types, field access via getelementptr
- Type conversions for the new types

### 6. Update Symbol Table (symbol_table.rs)

Add support for storing struct definitions and their field information.

## Testing Strategy

### Unit Tests
- Test lexer tokenizes new syntax correctly
- Test parser creates correct AST nodes
- Test type checker validates arrays/structs
- Test codegen produces valid LLVM IR

### Integration Tests
- Test complete programs using arrays and structs
- Test error cases (type mismatches, out of bounds)
- Test integration with existing features (functions, loops)

## Example Implementation

```rust
// Array usage
let numbers = [1, 2, 3, 4, 5];
söyle numbers[0];  // Outputs: 1

// Struct definition and usage
Öğrenci { isim: metin, yaş: tamsayı };

let student = Öğrenci { isim: "Ahmet", yaş: 20 };
söyle student.isim;  // Outputs: Ahmet
student.yaş = 21;
```

## Validation

Run the test suite:
```bash
cargo test
cargo clippy
```

Ensure all existing tests pass and new tests validate the data structures functionality.

## Next Steps

After implementation, run `/speckit.tasks` to generate detailed implementation tasks, then proceed with TDD development following the constitution's workflow.
