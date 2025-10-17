# otağ Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-10-17

## Constitution

read and follow the [constitution](../.specify/memory/constitution.md)

## Active Technologies
- Rust 1.75 (or latest stable) + pest, pest_derive (parsing), logos (lexing), clap (CLI) (002-add-control-flow)
- N/A (interpreter-based, no persistent storage) (002-add-control-flow)
- Rust 1.75 + pest, pest_derive, logos, clap, LLVM (003-add-functions)
- N/A (compiler produces executable code) (004-add-data-structures)

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
- 004-add-data-structures: Added Rust 1.75 (or latest stable) + LLVM backend for code generation
- 003-add-functions: Added Rust 1.75 + pest, pest_derive, logos, clap, LLVM
- 002-add-control-flow: Added Rust 1.75 (or latest stable) + pest, pest_derive (parsing), logos (lexing), clap (CLI)

  code generation

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
