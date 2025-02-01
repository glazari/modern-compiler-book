pub mod ast;
pub mod lexer;
pub mod grammar; 

use lexer::Token;
use logos::Logos;

pub fn main() {
    let example_prog = "asf 
bht kdi ";"111 
.,(){}[]<>:;*+-/=
while for to break let in end funciton var type array if then else do of nil
-32
\"string\"
(* comment *)
\"string with \\\"escaped quotes\"


\"\"
";
    println!("Running tiger.rs");
    println!("Program: '{}'", example_prog);

    let lexer = Token::lexer(example_prog)
        .spanned()
        .map(|(tok, span)| match tok {
            Err(_) => Err("Parse error".to_string()),
            Ok(t) => Ok((span.start, t, span.end)),
        });

    let parser = grammar::ExprParser::new();

    let expr = parser.parse(lexer).expect("Failed to parse");
    println!("Result: {}", expr.to_string());




}
