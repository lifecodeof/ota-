use crate::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Syntax,
    Semantic,
    Runtime,
}

#[derive(Debug, Clone)]
pub struct OtagError {
    pub error_type: ErrorType,
    pub message: String,
    pub location: Location,
    pub suggestions: Vec<String>,
}

impl OtagError {
    pub fn new(error_type: ErrorType, message: String, location: Location) -> Self {
        Self {
            error_type,
            message,
            location,
            suggestions: Vec::new(),
        }
    }

    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }

    pub fn syntax(message: String, location: Location) -> Self {
        Self::new(ErrorType::Syntax, message, location)
    }

    pub fn semantic(message: String, location: Location) -> Self {
        Self::new(ErrorType::Semantic, message, location)
    }

    pub fn runtime(message: String, location: Location) -> Self {
        Self::new(ErrorType::Runtime, message, location)
    }
}

impl std::fmt::Display for OtagError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.location, self.message)?;
        if !self.suggestions.is_empty() {
            write!(f, "\nÖneriler:")?;
            for suggestion in &self.suggestions {
                write!(f, "\n  - {}", suggestion)?;
            }
        }
        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, OtagError>;
