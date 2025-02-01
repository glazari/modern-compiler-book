%{
open Ast
%}

%token <string> IDENTIFIER
%token <int> NUM
%token <string> PUNCTUATION
%token <string> STRING
%token EOF
%type <expr> main
%start main
%%

main:
    expr EOF { $1 }
;

expr:
  tok { $1 }
  | expr tok { 
            match $1 with 
            | List l -> List(l @ [$2])
            | other -> List([other; $2])
            }
  
                
;
tok:
  IDENTIFIER {  
    match keyword_of_string $1 with
    | Some k -> Keyword(k)
    | None -> Iden($1)
  }
  | NUM { Num($1) }
  | PUNCTUATION { Punct( punc_of_string $1) }
  | STRING { StrLit($1) }


 


