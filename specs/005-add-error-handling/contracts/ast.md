# AST Contract: Error Handling

**Feature**: 005-add-error-handling  
**Date**: 2025-10-17  

## New AST Nodes

### TryCatch
```rust
pub struct TryCatch {
    pub try_block: Block,
    pub catch_var: String,
    pub catch_block: Block,
    pub finally_block: Option<Block>,
}
```

### Throw
```rust
pub struct Throw {
    pub expression: Expression,
}
```

### ErrorType
```rust
pub enum ErrorType {
    Syntax,
    Semantic,
    Runtime,
}
```

## Extended Enums
- Add TryCatch and Throw to Statement enum
- Add ErrorType to existing type system
