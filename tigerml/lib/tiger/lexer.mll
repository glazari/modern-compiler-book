{
  open Parser
}

rule token = parse
  | [' ' '\t' '\n'] { token lexbuf } (* skip whitespace *)
  | ['a'-'z']+ as lxm { IDENTIFIER lxm }
  | ['0'-'9']+ as lxm { NUM (int_of_string lxm) }
  | [',' ':' ';' '(' ')' '{' '}' '[' ']' '.' '-' '+' '*' '/' '=' '<' '>' '&' '|'] as lxm { PUNCTUATION (String.make 1 lxm) }
  | '\"' ([^'\"'] | '\\''\"')* '\"' as lxm { STRING (String.sub lxm 1 ((String.length lxm) - 2)) }
  (* TODO: fix comments, at the moment if you have 2 comments in the file, everything in between is considered a comment *)
  | '(''*' _* '*'')' { token lexbuf } (* skip comments *)
  | eof { EOF }
