%{
open Ast
%}

%token <string> IDENTIFIER
%token EOF
%type <expr> main
%start main
%%

main:
    expr EOF { $1 }
;

expr:
  IDENTIFIER { Iden($1) }
  | expr IDENTIFIER { 
            match $1 with 
            | Iden s -> List([Iden(s); Iden($2)])
            | List l -> List(l @ [Iden($2)])
            | Num n -> List([Num(n); Iden($2)])
            }
  
                
;

 


