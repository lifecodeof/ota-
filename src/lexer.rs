use logos::Logos;
use crate::error_reporting::{OtagError, Result};
use crate::location::Location;

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

    #[token("eğer")]
    Eger,

    #[token("yoksa")]
    Yoksa,

    #[token("döngü")]
    Dongu,

    #[token("için")]
    Icin,

    #[token("durdur")]
    Durdur,

    #[token("devam")]
    Devam,

    #[token("ise")]
    Ise,

    #[token("son")]
    Son,

    #[token("dan")]
    Dan,

    #[token("adım")]
    Adim,

    #[token("fonksiyon")]
    Fonksiyon,

    #[token("->")]
    Arrow,

    #[token("return")]
    Return,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(".")]
    Dot,

    #[token(":")]
    Colon,
}

pub fn lex(input: &str) -> Result<Vec<(Token, Location)>> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => {
                let span = lexer.span();
                let location = Location::new("<input>".to_string(), line, column);
                tokens.push((token, location));
                // Update position
                let token_text = &input[span.start..span.end];
                for c in token_text.chars() {
                    if c == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }
            }
            Err(_) => {
                let span = lexer.span();
                let location = Location::new("<input>".to_string(), line, column);
                return Err(OtagError::syntax("Geçersiz token".to_string(), location));
            }
        }
    }

    Ok(tokens)
}


