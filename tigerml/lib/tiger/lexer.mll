{
  open Parser
}

rule token = parse
  | [' ' '\t' '\n'] { token lexbuf } (* skip whitespace *)
  | ['a'-'z']+ as lxm { IDENTIFIER lxm }
  | eof { EOF }
