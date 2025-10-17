# Quickstart: Add Functions

## Overview

This guide outlines the steps to implement function support in the Otağ compiler.

## Prerequisites

- Rust 1.75 installed
- LLVM backend configured
- Existing parser, AST, and codegen modules

## Implementation Steps

1. **Extend Grammar** (`grammar.pest`):
   - Add rules for `function_definition` and `function_call`
   - Use Turkish keyword "fonksiyon" for definitions

2. **Update Lexer** (`lexer.rs`):
   - Add tokens for "fonksiyon", "->"

3. **Extend AST** (`ast.rs`):
   - Add `Function` and `Call` enum variants
   - Define structs for parameters

4. **Update Parser** (`parser.rs`):
   - Implement parsing for function definitions and calls

5. **Enhance Symbol Table** (`symbol_table.rs`):
   - Add function storage and lookup

6. **Extend Code Generator** (`codegen.rs`):
   - Generate LLVM IR for function definitions and calls

7. **Add Tests**:
   - Unit tests for parsing
   - Integration tests for execution

## Example Usage

```otag
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}

sonuç = topla(5, 3)
söyle sonuç  // Outputs 8
```

## Validation

Run `cargo test && cargo clippy` to ensure all tests pass and code quality is maintained.
