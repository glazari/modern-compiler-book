use logos::Logos;
use std::num::ParseIntError;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    #[regex(r"[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Int(i32),

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Times,

    #[token("/")]
    Div,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    EOL,
}
