# Grammar Contracts: Add Functions

## Function Definition

**Endpoint**: `function_definition`
**Pattern**: `fonksiyon ~ identifier ~ "(" ~ parameter_list? ~ ")" ~ ("->" ~ type)? ~ "{" ~ statement_list ~ "}"`

**Description**: Defines a reusable function with optional parameters and return type.

**Parameters**:
- `identifier`: Function name
- `parameter_list`: Comma-separated list of `identifier ":" type`
- `type`: Return type (optional)
- `statement_list`: Body statements

**Response**: AST Function node

## Function Call

**Endpoint**: `function_call`
**Pattern**: `identifier ~ "(" ~ argument_list? ~ ")"`

**Description**: Invokes a defined function with arguments.

**Parameters**:
- `identifier`: Function name
- `argument_list`: Comma-separated list of expressions

**Response**: AST Call node

**Validation**:
- Function must be defined
- Arguments must match parameter types and count
