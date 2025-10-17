# Lexer Contract

**Interface**: `Lexer`

**Purpose**: Tokenize Otağ source code into tokens for parsing.

**Input**: `&str` (UTF-8 source code)

**Output**: `Result<Vec<Token>, LexerError>`

**Token Types** (subset for feature):
- Identifier(String)
- Keyword(KeywordType) - e.g., Tanimla, Soyle
- Type(Type) - e.g., Tamsayi, Metin
- Literal(LiteralValue)
- Operator(Operator) - e.g., Assign, Plus
- Punctuation(Punct) - e.g., Apostrophe

**Error Handling**: Invalid characters, unterminated strings.

**Constraints**: Must handle Turkish UTF-8 characters correctly.
