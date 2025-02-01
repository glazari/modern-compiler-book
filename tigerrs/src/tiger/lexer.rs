
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Iden(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Int(i64),

    #[regex(r"[,:;(){}\[\].+-/*=<>|&]", |lex| lex.slice().to_string())]
    Punctuation(String),

    #[regex(r#""(:?[^"\\]|\\")*""#, clean_str_literal)]
    String(String),

    #[regex(r"\(\*", parse_comment)]
    Comment,

}


fn clean_str_literal(lex: &mut logos::Lexer<Token>) -> Option<String> {
    let s = lex.slice();
    let s = s[1..s.len()-1].replace("\\\"", "\"");
    Some(s)
}


fn parse_comment(lex: &mut logos::Lexer<Token>) -> logos::Skip {
    let mut len = 0;
    let mut depth = 1;
    let mut chars = lex.remainder().chars();
    while let Some(c) = chars.next() {
        len += c.len_utf8();
        match c {
            '(' => match chars.next() {
                Some('*') => {
                    depth += 1;
                    len += 1;
                }
                Some(_) => len += 1,
                _ => continue,
            },
            '*' => match chars.next() {
                Some(')') => {
                    depth -= 1;
                    len += 1;
                    if depth == 0 {
                        lex.bump(len + 1);
                        return logos::Skip;
                    }
                }
                Some(_) => len += 1,
                _ => continue,
            }
            _ => continue,
        }
    }
    lex.bump(len);
    logos::Skip
}
