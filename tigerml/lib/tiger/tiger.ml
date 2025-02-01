let example_prog = "asf 
bht kdi 111
.,(){}[]<>:;*+-/=
while for to break let in end funciton var type array if then else do of nil
-32
\"string\"
(* comment *)
\"string with \\\"escaped quotes\"


\"\"
"

let main () =
  Printf.printf "Running tigerml\n";
  Printf.printf "Program: %s\n" example_prog;
  let lexbuf = Lexing.from_string example_prog in
  let result = Parser.main Lexer.token lexbuf in
  Printf.printf "Result: %s\n"  (Ast.to_string result)
