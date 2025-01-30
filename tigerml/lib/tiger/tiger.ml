let example_prog = "asf bht kdi"

let main () =
  Printf.printf "Running tigerml\n";
  Printf.printf "Program: %s\n" example_prog;
  let lexbuf = Lexing.from_string example_prog in
  let result = Parser.main Lexer.token lexbuf in
  Printf.printf "Result: %s\n"  (Ast.to_string result)
