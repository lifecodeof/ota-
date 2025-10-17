# Data Model: Add Functions

## Entities

### Function

- **name**: String (unique identifier for the function)
- **parameters**: List of Parameter (input values)
- **return_type**: Type (output type, or void)
- **body**: List of Statement (executable code block)

**Relationships**:
- Function has many Parameters
- Function has one return_type
- Function has many Statements in body

**Validation Rules**:
- Name must be a valid identifier (alphanumeric + underscore, starts with letter or underscore, UTF-8 supported)
- Parameters must have unique names within the function
- Return type must be a supported type or void
- Body must contain valid statements

**State Transitions**: N/A (static definition)

### Parameter

- **name**: String (parameter identifier)
- **type**: Type (parameter data type)

**Relationships**:
- Belongs to Function

**Validation Rules**:
- Name must be valid identifier
- Type must be supported

### Type

Existing types from the language: integer, string, float, boolean, etc.

**Note**: No new types added for this feature.
