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

    pub fn type_mismatch(expected: &str, found: &str, location: Location) -> Self {
        Self::semantic(format!("Tür uyumsuzluğu: {} bekleniyordu, {} bulundu", expected, found), location)
    }

    pub fn undefined_variable(name: &str, location: Location) -> Self {
        Self::semantic(format!("Tanımlanmamış değişken: {}", name), location)
    }

    pub fn division_by_zero(location: Location) -> Self {
        Self::runtime("Sıfıra bölme hatası".to_string(), location)
    }
}

impl std::fmt::Display for OtagError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_kind = match self.error_type {
            ErrorType::Syntax => "Sözdizimi Hatası",
            ErrorType::Semantic => "Anlamsal Hata",
            ErrorType::Runtime => "Çalışma Zamanı Hatası",
        };
        
        writeln!(f, "\n{}: {}", error_kind, self.message)?;
        writeln!(f, "  --> {}", self.location)?;
        
        if !self.suggestions.is_empty() {
            writeln!(f, "\nÖneriler:")?;
            for suggestion in &self.suggestions {
                writeln!(f, "  • {}", suggestion)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for OtagError {}

pub type Result<T> = std::result::Result<T, OtagError>;
