# Data Model: Error Handling

**Feature**: 005-add-error-handling  
**Date**: 2025-10-17  

## Entities

### Error
- **Attributes**: type, message, location, suggestions
- **Relationships**: Belongs to compilation/runtime context

### ErrorType
- **Attributes**: category (syntax, semantic, runtime)
- **Relationships**: Has many errors

### Location
- **Attributes**: file, line, column
- **Relationships**: Referenced by errors

## Relationships
- Errors have one Location
- Errors belong to one ErrorType
- ErrorType categorizes multiple Errors

## Validation
- Location must have valid file path
- Error message must be non-empty
- ErrorType must be defined enum value
