# Research: Add Functions

## Performance Goals

**Decision**: Compilation time for programs with functions should be under 500ms for programs up to 1000 lines, with function call overhead under 10% compared to inline code.

**Rationale**: Based on benchmarks of similar small language compilers (e.g., Chibi Scheme, Lua), this provides acceptable performance for development and small applications while prioritizing simplicity over optimization per constitution.

**Alternatives Considered**:
- Stricter <100ms: Rejected as premature optimization; LLVM compilation is inherently slow for small programs.
- No limits: Rejected as could lead to poor user experience.

## Constraints

**Decision**: Memory usage during compilation limited to 50MB peak, no external dependencies beyond LLVM, functions must be statically analyzable at compile time.

**Rationale**: Aligns with cross-platform compatibility and safety principles; static analysis ensures compile-time error catching.

**Alternatives Considered**:
- Higher memory (100MB): Rejected to maintain lightweight footprint.
- Dynamic functions: Rejected as violates safety and simplicity.

## Scale/Scope

**Decision**: Support up to 100 functions per program, each with up to 10 parameters, nesting depth up to 3 levels.

**Rationale**: Sufficient for modular programming in small to medium programs; matches typical beginner-friendly language limits.

**Alternatives Considered**:
- Unlimited: Rejected to avoid complexity in symbol table and codegen.
- Lower limits (10 functions): Rejected as insufficient for modularity.
