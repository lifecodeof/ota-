# Data Model: Add Data Structures

**Feature**: Add Data Structures  
**Date**: 2025-10-17  

## Entities

### Array

**Description**: Represents a fixed-size collection of elements of the same type, accessible by index.

**Fields**:
- `element_type`: Type - The type of all elements in the array (must be basic types: integer, string, float, boolean)
- `elements`: Vec<Expression> - The expressions that initialize the array elements
- `size`: usize - The number of elements (inferred from elements vector)

**Relationships**:
- Each element in `elements` must evaluate to `element_type`
- Arrays can be assigned to variables and passed to functions
- Arrays can be used in control flow and output operations

**Validation Rules**:
- All elements must be of the same type (`element_type`)
- Array size is determined at compile time
- Index access must be within bounds (0 to size-1)
- Empty arrays are allowed but must specify element type implicitly or explicitly

**State Transitions**: None (immutable after creation)

### Struct

**Description**: Represents a custom data type with named fields of potentially different types.

**Fields**:
- `name`: String - The struct type name (used for declarations and instantiations)
- `fields`: Vec<Field> - The ordered list of fields in the struct

**Field Structure**:
- `name`: String - Field identifier (must be unique within struct)
- `field_type`: Type - The type of the field (basic types or other structs)

**Relationships**:
- Structs can contain other structs (nested structs)
- Structs can be assigned to variables and passed to functions
- Struct fields can be accessed and modified individually
- Structs can be used in control flow and output operations

**Validation Rules**:
- Field names must be unique within the struct
- Field types must be valid (basic types or defined structs)
- Struct instantiation must provide values for all fields
- Field access uses dot notation with exact field names

**State Transitions**: None (fields can be modified individually but struct definition is immutable)

### Variable (Extended)

**Description**: Extended to support array and struct types.

**Additional Fields**:
- `var_type`: Type - Now includes Array(type) and Struct{name}

**Validation Rules** (Extended):
- Assignment must match variable type
- For arrays: assigned value must be array literal or compatible array
- For structs: assigned value must be struct literal of same type

## Data Flow

1. **Declaration**: Variables declared with array/struct types
2. **Initialization**: Arrays/structs created via literals or expressions
3. **Usage**: Accessed via indexing/dot notation in expressions
4. **Operations**: Passed to functions, used in control flow, output to console
5. **Validation**: Type checking ensures safety at compile time
