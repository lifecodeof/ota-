# Grammar Contracts: Data Structures

**Feature**: Add Data Structures  
**Date**: 2025-10-17  

## Array Syntax

### Declaration
```
array_literal ::= "[" [expression ("," expression)*] "]"
```

**Examples**:
- `[]` - Empty array (type inferred from context)
- `[1, 2, 3]` - Integer array
- `["merhaba", "dünya"]` - String array

### Access
```
array_access ::= expression "[" expression "]"
```

**Examples**:
- `dizi[0]` - Access first element
- `matris[i][j]` - Nested access

### Assignment
```
array_assignment ::= array_access "=" expression
```

**Examples**:
- `dizi[1] = 42`

## Struct Syntax

### Definition
```
struct_definition ::= identifier "{" field_definition ("," field_definition)* "}"
field_definition ::= identifier ":" type
```

**Examples**:
- `Öğrenci { isim: metin, yaş: tamsayı }`

### Instantiation
```
struct_literal ::= identifier "{" field_assignment ("," field_assignment)* "}"
field_assignment ::= identifier ":" expression
```

**Examples**:
- `Öğrenci { isim: "Ahmet", yaş: 20 }`

### Access
```
struct_access ::= expression "." identifier
```

**Examples**:
- `öğrenci.isim` - Access field
- `nokta.x` - Access nested if applicable

### Assignment
```
struct_assignment ::= struct_access "=" expression
```

**Examples**:
- `öğrenci.yaş = 21`

## Type System Extensions

### Array Types
```
array_type ::= "[" type "]"  # Future extension
# Currently inferred from literals
```

### Struct Types
```
struct_type ::= identifier  # Reference to defined struct
```

## Integration Contracts

### With Variables
- Arrays/structs can be assigned to variables
- Variable declarations can specify array/struct types (future)

### With Functions
- Arrays/structs can be passed as parameters
- Structs can be returned from functions
- Arrays/structs can be used in expressions

### With Control Flow
- Arrays/structs can be used in conditions (truthiness)
- Arrays can be iterated in loops
- Struct fields can be used in conditions

### With Output
- Arrays/structs can be printed
- Nested structures display appropriately
