pub mod lexer;
pub mod ast;
pub mod grammar;

use lexer::Token;
use logos::Logos;

pub fn main() {
    let example_prog = "3 * 5 + 4 * -6";
    println!("Running calc.rs");
    println!("Program: {}", example_prog);

    let lexer = Token::lexer(example_prog)
        .spanned()
        .map(|(tok, span)| match tok {
            Ok(Token::Error) => Err("Lexer error".to_string()),
            Err(_) => Err("Parse error".to_string()),
            Ok(t) => Ok((span.start, t, span.end)),
        });

    let parser = grammar::ExprParser::new();

    let expr = parser.parse(lexer).expect("Failed to parse");
    println!("Result: {}", expr.to_string());
}
