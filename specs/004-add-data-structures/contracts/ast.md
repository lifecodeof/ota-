# AST Contracts: Data Structures

**Feature**: Add Data Structures  
**Date**: 2025-10-17  

## New AST Nodes

### ArrayLiteral
```rust
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
    pub element_type: Option<Type>, // Inferred if None
}
```

**Responsibilities**:
- Represent array creation with literal values
- Validate all elements are same type
- Provide type information for codegen

### StructLiteral
```rust
pub struct StructLiteral {
    pub struct_name: String,
    pub fields: Vec<FieldAssignment>,
}

pub struct FieldAssignment {
    pub name: String,
    pub value: Expression,
}
```

**Responsibilities**:
- Represent struct instantiation
- Validate field names match struct definition
- Ensure all fields are provided

### ArrayAccess
```rust
pub struct ArrayAccess {
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}
```

**Responsibilities**:
- Represent element access by index
- Validate index is integer type
- Generate bounds checking

### StructAccess
```rust
pub struct StructAccess {
    pub struct_expr: Box<Expression>,
    pub field_name: String,
}
```

**Responsibilities**:
- Represent field access by name
- Validate field exists in struct type
- Provide field type information

### StructDefinition
```rust
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
}

pub struct FieldDefinition {
    pub name: String,
    pub field_type: Type,
}
```

**Responsibilities**:
- Define struct types
- Validate field name uniqueness
- Register type in symbol table

## Extended AST Nodes

### Type Enum (Extended)
```rust
pub enum Type {
    // Existing types...
    Array(Box<Type>),        // Array of specific type
    Struct(String),          // Struct by name
    // ...
}
```

### Expression Enum (Extended)
```rust
pub enum Expression {
    // Existing expressions...
    ArrayLiteral(ArrayLiteral),
    StructLiteral(StructLiteral),
    ArrayAccess(ArrayAccess),
    StructAccess(StructAccess),
    // ...
}
```

### Statement Enum (Extended)
```rust
pub enum Statement {
    // Existing statements...
    StructDefinition(StructDefinition),
    // ...
}
```

## Type Checking Contracts

### Array Type Checking
- All elements must be same type
- Index expressions must be integers
- Access bounds validated at compile time

### Struct Type Checking
- Struct literals must match definition exactly
- Field names must exist in definition
- Field types must match definition

## Codegen Contracts

### Array Codegen
- Allocate space on stack for fixed-size arrays
- Generate element access via getelementptr
- Handle empty arrays appropriately

### Struct Codegen
- Create LLVM struct type with named fields
- Generate field access via getelementptr
- Handle nested structs recursively

## Symbol Table Contracts

### Struct Registration
- Struct definitions added to global symbol table
- Field information stored for validation
- Type information available for expressions

### Variable Type Extensions
- Variables can have Array or Struct types
- Type checking uses extended type system
