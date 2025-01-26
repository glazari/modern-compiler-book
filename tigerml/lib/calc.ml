let example_prog = "3 * 5 + 4 * 6\n" (* 39 *)

let main () =
  Printf.printf "Running calc.ml\n";
  Printf.printf "Program: %s\n" example_prog;
  try
    let lexbuf = Lexing.from_string example_prog in
    while true do
      let result = Parser.main Lexer.token lexbuf in
      Printf.printf "Result: %s\n" (Ast_calc.to_string result)
    done
    (* on error, return *)
  with Lexer.Eof -> ()
