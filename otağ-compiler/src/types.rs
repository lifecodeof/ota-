#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Tamsayi,    // integer
    Metin,      // string
    Ondalikli,  // float
    Mantiksal,  // boolean
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Int(i32),
    String(String),
    Float(f64),
    Bool(bool),
}
