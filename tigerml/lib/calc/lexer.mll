{
  open Parser
  exception Eof
}

rule token = parse
  | [' ' '\t' ] { token lexbuf } (* skip whitespace *)
  | ['\n'] { EOL } 
  | ['0'-'9']+ as lxm { INT (int_of_string lxm) } 
  | '+' { PLUS }
  | '-' { MINUS }
  | '*' { TIMES }
  | '/' { DIV }
  | '(' { LPAREN }
  | ')' { RPAREN }
  | eof { raise Eof }
 
