#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(file: String, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }

    pub fn unknown() -> Self {
        Self {
            file: "<unknown>".to_string(),
            line: 0,
            column: 0,
        }
    }

    pub fn from_position(input: &str, position: usize, file: String) -> Self {
        let before = &input[..position];
        let line = before.chars().filter(|&c| c == '\n').count() + 1;
        let column = before.chars().rev().take_while(|&c| c != '\n').count() + 1;
        Self { file, line, column }
    }

    pub fn from_pest_span(input: &str, span: &pest::Span, file: String) -> Self {
        Self::from_position(input, span.start(), file)
    }

    pub fn advance_column(&mut self, by: usize) {
        self.column += by;
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}
