# Research Findings: Add Variable Output

**Feature**: Add Variable Output  
**Date**: 2025-10-17  
**Researcher**: AI Assistant  

## Performance Goals

**Decision**: Compilation time under 1 second for programs up to 100 lines, execution with reasonable speed for basic operations (no specific benchmarks yet).  
**Rationale**: Aligns with beginner-friendly priority - fast feedback for learning. Based on typical compiler expectations for small languages.  
**Alternatives Considered**: Stricter goals (<500ms) rejected as premature optimization; looser goals (>5s) would violate simplicity principle.

## Lexer Implementation

**Decision**: Use `logos` crate for tokenization, with custom UTF-8 handling for Turkish characters.  
**Rationale**: `logos` is fast, supports Unicode, and integrates well with Rust. Allows defining tokens for Turkish keywords like 'tanımla', 'söyle'.  
**Alternatives Considered**: Hand-rolled lexer (too error-prone); `nom` (better for parsing than lexing); regex-based (slower).

## Parser Implementation

**Decision**: Use `pest` crate for grammar-based parsing of Otağ syntax.  
**Rationale**: Declarative grammar definition matches the EBNF spec, easy to maintain and extend. Supports UTF-8 identifiers.  
**Alternatives Considered**: `nom` parser combinators (more complex for grammar changes); hand-rolled recursive descent (harder to maintain).

## UTF-8 and Turkish Character Handling

**Decision**: Leverage Rust's native UTF-8 string support; validate Turkish characters in lexer using Unicode categories.  
**Rationale**: Rust strings are UTF-8 by default, no additional crates needed. Ensures proper handling of i/İ, ğ, etc. in variable names.  
**Alternatives Considered**: ICU4X crate (overkill for basic needs); custom normalization (unnecessary complexity).

## LLVM Code Generation

**Decision**: Use `inkwell` crate for LLVM IR generation, focusing on alloca for variables and printf for output.  
**Rationale**: `inkwell` provides safe Rust bindings to LLVM, supports cross-platform compilation. Straightforward for basic variable storage and I/O.  
**Alternatives Considered**: Direct LLVM C API (unsafe, complex); Cranelift (not LLVM-based, violates constitution); no codegen (interpreter-only, but constitution specifies LLVM).

## Type System Basics

**Decision**: Implement basic types (tamsayı=int, metin=string, etc.) as LLVM types with runtime type checking.  
**Rationale**: Matches spec requirements, ensures safety. Start simple, extend later.  
**Alternatives Considered**: Dynamic typing (violates safety principle); compile-time only (too restrictive for beginners).
