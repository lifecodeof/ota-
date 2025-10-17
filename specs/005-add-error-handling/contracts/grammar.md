# Grammar Contract: Error Handling

**Feature**: 005-add-error-handling  
**Date**: 2025-10-17  

## New PEG Rules

### Try-Catch-Finally
```
try_catch = { "dene" ~ block ~ "yakala" ~ identifier ~ block ~ ("sonunda" ~ block)? }
```

### Throw Statement
```
throw_stmt = { "fırlat" ~ expression }
```

### Error Declaration
```
error_decl = { "hata" ~ identifier ~ "=" ~ string_literal }
```

## Updated Rules
- Extend statement rule to include try_catch and throw_stmt
- Update expression rule for error handling
