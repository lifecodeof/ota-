# Otağ Programming Language

A Turkish-localized programming language compiler implemented in Rust, focusing on simplicity, readability, and beginner-friendliness.

## Overview

Otağ is designed with Turkish localization as a core principle, providing an intuitive programming experience for Turkish speakers. The language emphasizes natural syntax while maintaining strong type safety and performance.

## Features

- **Turkish Keywords**: Natural Turkish syntax (`tanımla`, `söyle`, `fonksiyon`, etc.)
- **Type Safety**: Compile-time type checking
- **Functions**: Define and call reusable code blocks
- **Module System**: Import and reuse code from other files with `kullan` keyword
- **UTF-8 Support**: Full Turkish character support (ğ, ü, ş, ö, ç, ı)
- **Simple Syntax**: Beginner-friendly, readable code
- **Cross-Platform**: Works on Windows, Linux, and macOS

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/your-repo/otağ.git
cd otağ

# Build the compiler
cargo build --release
```

### Basic Usage

Create a file `hello.otağ`:

```otağ
# Variable declarations
yaş'ı tamsayı olarak tanımla
yaş = 25
mesaj = "Merhaba Otağ!"

# Output
söyle yaş
söyle mesaj
```

Run it:

```bash
cargo run hello.otağ
```

Output:
```
25
Merhaba Otağ!
```

## Language Syntax

### Variable Declaration

```otağ
# Declare variables with Turkish possessive
x'ı tamsayı olarak tanımla      # integer
isim = "Merhaba"                # string
puan'ı ondalıklı olarak tanımla  # float
durum'u mantıksal olarak tanımla # boolean
```

### Data Types

- `tamsayı` - Integer (i32)
- `metin` - String
- `ondalıklı` - Float (f64)
- `mantıksal` - Boolean

### Expressions

```otağ
# Basic arithmetic
sonuç = x + 5
oran = puan * 2.0

# Output expressions
söyle x + y
söyle isim + " Dünya"
```

### Output

```otağ
söyle değişken    # Print variable
söyle ifade       # Print expression result
```

### Functions

```otağ
# Define a function with parameters and return type
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}

# Call a function
sonuç = topla(5, 3)
söyle sonuç  # Outputs 8
```

### Module System

Import and reuse code from other files:

`matematik.otağ`:
```otağ
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}
```

`main.otağ`:
```otağ
kullan "matematik.otağ"

sonuç'ı tamsayı olarak tanımla
sonuç = topla(5, 3)
söyle sonuç  # Outputs 8
```

## Examples

See the `examples/` directory for complete examples:

- `basic.otağ` - Simple variable declaration
- `variable.otağ` - Variable usage
- `types.otağ` - All data types
- `expressions.otağ` - Arithmetic expressions
- `import-test.otağ` - Module importing example
- `nested-import.otağ` - Nested module imports

## Development

### Prerequisites

- Rust 1.75 or later
- Cargo

### Building

```bash
cargo build          # Debug build
cargo build --release # Optimized build
```

### Testing

```bash
cargo test           # Run all tests
cargo test --doc     # Run documentation tests
```

### Code Quality

```bash
cargo clippy         # Lint code
cargo fmt            # Format code
```

## Project Structure

```
otağ/
├── src/
│   ├── main.rs       # CLI entry point
│   ├── lexer.rs      # Tokenization
│   ├── parser.rs     # AST parsing
│   ├── ast.rs        # Abstract Syntax Tree
│   ├── codegen.rs    # Code generation (interpreter)
│   ├── types.rs      # Type system
│   └── symbol_table.rs # Symbol management
├── examples/         # Example programs
├── specs/           # Feature specifications
└── Cargo.toml       # Project configuration
```

## Architecture

Otağ uses a traditional compiler pipeline:

1. **Lexer**: Tokenizes source code
2. **Parser**: Builds Abstract Syntax Tree (AST)
3. **Semantic Analyzer**: Performs type checking and validation
4. **Code Generator**: Currently uses an interpreter for execution

Future versions will include LLVM-based code generation for native performance.

## Testing and Library API

Otağ provides a powerful in-memory testing API that allows you to test Otağ programs without creating physical files. This is particularly useful for unit testing and integration testing.

### Using the In-Memory API

Add `otag` as a dependency in your test files:

```rust
use otag::OtagRuntime;

#[test]
fn test_simple_program() {
    let source = r#"
x'ı tamsayı olarak tanımla
x = 42
söyle x
"#;
    
    let result = OtagRuntime::execute_inline(source);
    assert!(result.is_ok());
}
```

### Testing with Virtual Imports

```rust
use otag::OtagRuntime;

#[test]
fn test_with_modules() {
    let mut runtime = OtagRuntime::new();
    
    // Add virtual source files
    runtime.add_source("math.otağ", r#"
fonksiyon topla(a: tamsayı, b: tamsayı) -> tamsayı {
    return a + b
}
"#);
    
    runtime.add_source("main.otağ", r#"
kullan "math.otağ"

sonuç'ı tamsayı olarak tanımla
sonuç = topla(5, 3)
söyle sonuç
"#);
    
    let result = runtime.execute("main.otağ");
    assert!(result.is_ok());
}
```

See `tests/test_in_memory.rs` for more comprehensive examples.

## Contributing

1. Follow the [Constitution](.specify/memory/constitution.md)
2. Use Test-Driven Development (TDD)
3. Ensure all tests pass
4. Update documentation as needed

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Parser powered by [Pest](https://pest.rs/)
- Inspired by the need for Turkish-localized programming languages</content>
<parameter name="filePath">d:\uni\senior\otağ\README.md
