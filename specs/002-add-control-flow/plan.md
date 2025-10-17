# Implementation Plan: Control Flow Statements

**Branch**: `002-add-control-flow` | **Date**: 2025-10-17 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-add-control-flow/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add Turkish-localized control flow statements (eğer/yoksa for if/else, döngü for while loops, için for for loops, durdur/devam for break/continue) to the Otağ programming language. Implementation will extend the existing interpreter-based compiler with new AST nodes, parser rules, and execution logic following TDD principles.

## Technical Context

**Language/Version**: Rust 1.75 (or latest stable)  
**Primary Dependencies**: pest, pest_derive (parsing), logos (lexing), clap (CLI)  
**Storage**: N/A (interpreter-based, no persistent storage)  
**Testing**: cargo test (unit tests alongside source code)  
**Target Platform**: Cross-platform (Windows, Linux, macOS)  
**Project Type**: Single compiler project (interpreter-based)  
**Performance Goals**: Educational performance (minimal requirements acceptable)  
**Constraints**: Full UTF-8 support with Turkish character handling, Turkish localization priority  
**Scale/Scope**: Language syntax extension (control flow statements)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Compliance ✅
- **I. Turkish Localization Priority**: ✅ Feature uses Turkish keywords (eğer, yoksa, döngü, için, durdur, devam, dan, adım, ise, son)
- **II. Simplicity and Readability**: ✅ Control flow syntax designed to be intuitive ("eğer ... ise ... yoksa ... son")
- **III. Feature Implementation Priority**: ✅ Following TDD workflow for rapid, testable development
- **IV. Safety and Reliability**: ✅ Type-safe conditions, predictable execution, infinite loop protection
- **V. Extensibility and Modularity**: ✅ Adding to existing modular AST/parser/codegen structure

### Technical Standards Compliance ✅
- **UTF-8 Support**: ✅ Turkish characters supported in all keywords
- **Standard Library**: ✅ Turkish-specific control flow utilities
- **Cross-platform**: ✅ Rust provides Windows/Linux/macOS compatibility
- **Rust + LLVM Backend**: ⚠️ Currently using interpreter (consistent with previous feature), LLVM backend planned for future optimization

### Development Workflow Compliance ✅
- **TDD**: ✅ Design includes comprehensive unit tests and acceptance scenarios
- **Testing Standards**: ✅ Unit tests alongside source, integration tests, cargo test && cargo clippy
- **Logging Standards**: ✅ Will use structured logging if needed for debugging

### Governance Compliance ✅
- **Open Source**: ✅ Following established contribution patterns
- **Code Reviews**: ✅ Changes will be reviewed
- **CI/CD**: ✅ Automated testing pipeline
- **Spec Maintenance**: ✅ Feature spec created and maintained

**Gate Status**: ✅ PASS - No violations requiring justification. Design maintains constitution compliance.

## Project Structure

### Documentation (this feature)

```
specs/002-add-control-flow/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```
otağ-compiler/
├── src/
│   ├── main.rs          # CLI entry point (clap integration)
│   ├── ast.rs           # Abstract Syntax Tree (extend with control flow nodes)
│   ├── parser.rs        # Pest grammar parser (add control flow rules)
│   ├── codegen.rs       # Interpreter (add control flow execution)
│   ├── lexer.rs         # Logos lexer (add control flow keywords)
│   ├── symbol_table.rs  # Symbol management (extend for loop variables)
│   └── types.rs         # Type system (extend if needed)
├── examples/            # Test programs
│   ├── basic.otağ       # Existing examples
│   ├── expressions.otağ # Existing examples
│   ├── types.otağ       # Existing examples
│   ├── variable.otağ    # Existing examples
│   ├── control-flow.otağ    # NEW: Control flow examples
│   └── loops.otağ           # NEW: Loop examples
└── tests/               # Integration tests
    └── control_flow.rs  # NEW: Control flow integration tests
```

**Structure Decision**: Single compiler project structure maintained. Control flow feature extends existing modular architecture (lexer/parser/AST/codegen) without requiring new top-level directories.

## Complexity Tracking

*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |

