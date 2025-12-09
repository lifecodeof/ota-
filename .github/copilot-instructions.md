# Otağ Development Guidelines

## About the Project

Otağ is a beginner-friendly programming language designed for Turkish speakers. It features Turkish keywords, natural syntax, and comprehensive error messages in Turkish to make programming more accessible.

## Core Principles

### 1. Localization
- **User-facing strings MUST be in Turkish**: Error messages, compiler output, warnings, help text, and any text visible to end users
- **Internal code MUST be in English**: Variable names, function names, comments, log messages, and developer-facing documentation
- **Example**:
  ```rust
  // Good: Internal variable in English, error message in Turkish
  let variable_name = "x";
  return Err(OtagError::semantic(format!("Tanımlanmamış değişken: {}", variable_name), location));
  
  // Bad: Mixing Turkish in internal code
  let değişken_adı = "x";  // Don't do this
  ```

### 2. Beginner-Friendly Design
- Keep syntax simple and intuitive
- Provide clear, helpful error messages with suggestions when possible
- Follow natural Turkish grammar for keywords (e.g., possessive suffixes: `x'ı`, `yaş'ı`)
- Error messages should guide users toward solutions, not just state problems
- Take inspiration from well-known languages (Python, Rust, Go) for clear patterns

### 3. Language Syntax Standards
- Turkish keywords: `tanımla`, `söyle`, `fonksiyon`, `eğer`, `yoksa`, `döngü`, `kullan`, `return`
- Type names: `tamsayı` (integer), `metin` (string), `ondalıklı` (float), `mantıksal` (boolean)
- Natural possessive syntax: `değişken'ı tamsayı olarak tanımla`
- Full UTF-8 support for Turkish characters (ğ, ü, ş, ö, ç, ı)

## Development Workflow

### Testing Requirements
**ALWAYS utilize automated testing** before submitting changes:

1. **Run all tests**:
   ```bash
   cargo test
   ```

2. **Run specific test modules**:
   ```bash
   cargo test --test test_imports
   cargo test parser::tests
   ```

3. **When adding new features**:
   - Write unit tests in the relevant module (e.g., `parser.rs`, `codegen.rs`)
   - Add integration tests in `tests/` directory if needed
   - Ensure tests cover edge cases and error conditions
   - Test with Turkish character input to ensure UTF-8 handling

### Code Quality Standards

1. **Linting** - Fix all clippy warnings:
   ```bash
   cargo clippy
   cargo clippy --fix  # Auto-fix when possible
   ```

2. **Formatting** - Use standard Rust formatting:
   ```bash
   cargo fmt
   ```

3. **Build verification**:
   ```bash
   cargo build          # Debug build
   cargo build --release  # Release build
   ```

### Making Changes

1. **Before making changes**:
   - Understand the existing code structure
   - Check existing tests to understand expected behavior
   - Run tests to ensure current state is working

2. **While developing**:
   - Write tests first (TDD approach preferred)
   - Keep changes minimal and focused
   - Ensure user-facing strings are in Turkish
   - Ensure internal code uses English

3. **Before committing**:
   - Run `cargo test` - all tests must pass
   - Run `cargo clippy` - no warnings allowed
   - Run `cargo fmt` - code must be formatted
   - Test with example programs in `examples/` directory
   - Verify error messages are clear and in Turkish

## Project Structure

```
otağ/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lexer.rs          # Tokenization
│   ├── parser.rs         # AST parsing with unit tests
│   ├── ast.rs            # Abstract Syntax Tree definitions
│   ├── codegen.rs        # Code generation/interpreter with tests
│   ├── types.rs          # Type system
│   ├── semantic.rs       # Semantic analysis
│   ├── symbol_table.rs   # Symbol management
│   ├── error_reporting.rs # Error types and formatting
│   └── location.rs       # Source location tracking
├── tests/
│   ├── integration/      # Integration tests
│   └── test_imports.rs   # Import system tests
└── examples/             # Example Otağ programs
```

## Error Handling Guidelines

- Use the `OtagError` type from `error_reporting.rs`
- Provide location information for all errors
- Add helpful suggestions when possible using `with_suggestions()`
- Error types: `Syntax`, `Semantic`, `Runtime`
- All error messages displayed to users MUST be in Turkish

Example:
```rust
OtagError::semantic(
    format!("Tanımlanmamış değişken: {}", var_name),
    location
).with_suggestions(vec![
    "Değişkeni kullanmadan önce tanımlayın".to_string(),
    format!("Örnek: {}'ı tamsayı olarak tanımla", var_name),
])
```

## Adding New Features

1. **Language features** (keywords, syntax):
   - Update lexer for new tokens
   - Update parser for new grammar rules
   - Update AST for new node types
   - Update codegen for execution
   - Add comprehensive tests
   - Update documentation in `docs/` and `README.md`

2. **Compiler features**:
   - Follow existing patterns in the codebase
   - Maintain separation between Turkish user interface and English internals
   - Add tests for new functionality
   - Update help text if adding CLI options

## References

- Follow Rust best practices and idioms
- Inspired by: Python (simplicity), Rust (safety), Go (clarity)
- Parser: Using Pest grammar (see grammar files)
- All contributions should align with beginner-friendly principles
