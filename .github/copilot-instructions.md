# otağ Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-10-17

## Constitution

read and follow the [constitution](../.specify/memory/constitution.md)

## Active Technologies
- Rust 1.75 (or latest stable) + pest, pest_derive (parsing), logos (lexing), clap (CLI) (002-add-control-flow)
- N/A (interpreter-based, no persistent storage) (002-add-control-flow)
- Rust 1.75 + pest, pest_derive, logos, clap, LLVM (003-add-functions)
- N/A (compiler produces executable code) (004-add-data-structures)
- [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION] + [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION] (005-add-error-handling)
- [if applicable, e.g., PostgreSQL, CoreData, files or N/A] (005-add-error-handling)

- Rust 1.75 (or latest stable) + LLVM backend for code generation
  (001-add-variable-output)

## Project Structure

```
otağ-compiler/
  src/
  tests/
```

## Commands

cargo test; cargo clippy

## Code Style

Rust 1.75 (or latest stable): Follow standard conventions

## Recent Changes
- 005-005-add-error-handling: Added [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION] + [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION]
- 004-add-data-structures: Added Rust 1.75 (or latest stable) + LLVM backend for code generation
- 003-add-functions: Added Rust 1.75 + pest, pest_derive, logos, clap, LLVM

  code generation

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
