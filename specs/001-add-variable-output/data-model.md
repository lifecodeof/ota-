# Data Model: Add Variable Output

**Feature**: Add Variable Output  
**Date**: 2025-10-17  

## Entities

### Variable

**Description**: Represents a named storage location for data in Otağ programs.

**Fields**:
- `name`: String - Variable identifier (snake_case, UTF-8 characters, no leading digit)
- `var_type`: VariableType enum - Data type (tamsayı, metin, ondalıklı, mantıksal)
- `value`: Optional<VariableValue> - Current value (None if uninitialized)

**VariableType Enum**:
- Tamsayi (integer)
- Metin (string)
- Ondalikli (float)
- Mantiksal (boolean)

**VariableValue Enum** (tagged union):
- Int(i32)
- String(String)
- Float(f64)
- Bool(bool)

**Relationships**:
- None (standalone for basic feature)

**Validation Rules**:
- Name must match regex: `^[a-zğüşöçı][a-z0-9ğüşöçı_]*$` (snake_case, Turkish chars)
- Type must be one of the supported types
- Value type must match declared type (if assigned)

**State Transitions**:
- Uninitialized → Initialized (via assignment)
- Any state → Updated (via reassignment)

## AST Nodes (Implementation Model)

### VariableDeclaration
- `name`: String
- `var_type`: VariableType

### Assignment
- `name`: String
- `expression`: Expression (simple literals for now)

### OutputStatement
- `expression`: Expression (variable reference or literal)

### Expression (subset)
- Variable(String)
- Literal(VariableValue)

## Symbol Table

**Description**: Maps variable names to Variable entities within scope.

**Operations**:
- Insert: Add new variable declaration
- Lookup: Retrieve variable by name
- Update: Modify variable value

**Scope**: Global for basic feature (extend to local later)
