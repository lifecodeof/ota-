use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // skip whitespace
pub enum Token {
    #[token("tanımla")]
    Tanimla,

    #[token("söyle")]
    Soyle,

    #[token("tamsayı")]
    Tamsayi,

    #[token("metin")]
    Metin,

    #[token("ondalıklı")]
    Ondalikli,

    #[token("mantıksal")]
    Mantiksal,

    #[token("'")]
    Apostrophe,

    #[token("=")]
    Assign,

    #[regex(r"[a-zğüşöçı][a-z0-9ğüşöçı_]*")]
    Identifier,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    #[regex(r"\d+\.\d+")]
    FloatLiteral,

    #[regex(r"\d+")]
    IntLiteral,

    #[token("doğru")]
    True,

    #[token("yanlış")]
    False,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(tok) => tokens.push(tok),
            Err(_) => return Err(format!("Unexpected token at position {}", lexer.span().start)),
        }
    }

    Ok(tokens)
}
