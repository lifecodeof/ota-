<!--
Sync Impact Report:
- Version change: 1.2.2 → 1.2.3
- Modified sections: Development Workflow (added Logging Standards section)
- Templates requiring updates: none
- Follow-up TODOs: none
-->

# Otağ Programming Language Constitution

## Core Principles

### I. Turkish Localization Priority

The language must prioritize Turkish keywords, syntax, and documentation. All
core features must be accessible and intuitive for Turkish speakers.
Localization is not an afterthought but a core design principle.

### II. Simplicity and Readability

Syntax should be simple, readable, and expressive. Avoid unnecessary complexity;
prefer clear, concise constructs that are easy to understand and maintain.

### III. Feature Implementation Priority and Beginner-Friendliness

Prioritize rapid feature development and implementation over premature
optimization. The language should be close to natural language, especially
Turkish, to be beginner-friendly. For example, "x'i tamsayı olarak tanımla"
declares x as an integer type.

### IV. Safety and Reliability

Provide strong type safety, memory safety where possible, and predictable
behavior. Errors should be caught at compile time when feasible.

### V. Extensibility and Modularity

Support modular design, libraries, and extensions. Allow users to build upon the
language without modifying core.

## Technical Standards

- Full UTF-8 support with proper Turkish character handling (i, İ, ğ, etc.).
- Standard library must include Turkish-specific utilities.
- Cross-platform compatibility (Windows, Linux, macOS).
- Compiler implemented in Rust using LLVM backend.

## Development Workflow

IMPORTANT: Do not forget to mark tasks as done and commit changes after
completing each step.

### Test-Driven Development (TDD)

At Otağ, we embrace Test-Driven Development as our core development methodology.
TDD ensures code quality, maintainability, and correctness by writing tests
before implementation.

#### The TDD Workflow

Our standard development workflow follows this disciplined cycle:

1. **Test (fail)**: Write a failing test that defines the desired behavior
2. **Implement**: Write the minimal code to make the test pass
3. **Compile (fix)**: Run the compiler and fix any errors
4. **Test (success)**: Verify the test passes and all existing tests still work
5. **Lint (fix)**: Run linters and fix any style or quality issues
6. **Commit**: Commit the changes using git with a clear, descriptive message
7. **Mark as done**: Update task tracking to reflect completion

#### Why TDD?

- **Quality Assurance**: Tests serve as living documentation and prevent
  regressions
- **Design Improvement**: Writing tests first leads to better API design
- **Confidence**: Extensive test coverage enables fearless refactoring
- **Incremental Progress**: Small, testable steps prevent large, risky changes
- **Collaboration**: Tests provide clear specifications for team members

#### TDD in Practice

- All new features start with failing tests
- Tests are written at the appropriate level (unit, integration, end-to-end)
- Code coverage is monitored and maintained above acceptable thresholds
- Tests are treated as first-class citizens alongside production code
- Refactoring is done with confidence when tests provide safety nets

#### Tools and Commands

```bash
# Run all tests
cargo test

# Run linter
cargo clippy

# Run both (our standard check)
cargo test && cargo clippy
```

#### Test Organization

- Unit tests live alongside source code in the same files
- Integration tests in `tests/` directory
- End-to-end tests for complete workflows
- Benchmarks for performance-critical code

### Logging Standards

Proper logging is essential for debugging, monitoring, and maintaining the Otağ
compiler and associated tools. We enforce structured logging practices to ensure
consistency and professionalism.

#### Logging Guidelines

- **Use proper logging crates**: Prefer `log` with `env_logger` or `tracing`
  over `println!` and `eprintln!`
- **Structured logging**: Use appropriate log levels (error, warn, info, debug,
  trace)
- **Performance awareness**: Use appropriate log levels to avoid performance
  impact in release builds
- **Contextual information**: Include relevant context (file names, line
  numbers, variable values)

#### Log Levels

- `error!()`: Critical errors that prevent normal operation
- `warn!()`: Warnings that don't stop execution but indicate potential issues
- `info!()`: General information about program flow
- `debug!()`: Detailed information for debugging
- `trace!()`: Very detailed tracing information

#### Anti-patterns

Avoid these common logging mistakes:

- Using `println!()` or `eprintln!()` for diagnostic output
- Over-logging in hot code paths
- Inconsistent log formatting

#### Example Usage

```rust
use log::{error, warn, info, debug};

// Good: Structured logging with context
info!("Starting compilation for file: {}", filename);
debug!("Parsed AST: {:?}", ast);
error!("Failed to compile: {}", err);
warn!("Using deprecated feature");
trace!("Entering function foo with args: {:?}", args);

// Bad: Direct printing
eprintln!("Error occurred"); // Don't do this
```

- Open source development with community contributions.
- Code reviews required for all changes.
- Automated testing and CI/CD pipelines.
- Encourage usage of modern test frameworks and existing tools for comprehensive
  testing and quality assurance.

## Governance

Constitution supersedes all other practices. Amendments require community
consensus and documentation. All changes must verify compliance with principles.
Agents are responsible for committing their generated work and changes. The
language specification resides in [spec.md](spec/spec.md) and must be maintained
alongside constitution.md. Any additions of new features or fixes to syntax
logic errors must update [spec.md](spec/spec.md) accordingly.

**Version**: 1.2.3 | **Ratified**: 2025-10-17 | **Last Amended**: 2025-01-27
