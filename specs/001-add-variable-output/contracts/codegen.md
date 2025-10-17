# Code Generator Contract

**Interface**: `CodeGen`

**Purpose**: Generate LLVM IR from AST for compilation.

**Input**: `&Program`

**Output**: `Result<(), CodeGenError>`

**Operations**:
- Allocate variables on stack
- Store/load values
- Generate print calls for output
- Handle basic types (int, string, etc.)

**Error Handling**: Type errors, undefined variables.

**Constraints**: Cross-platform LLVM IR, safe memory operations.
