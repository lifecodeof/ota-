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


