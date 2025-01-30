%{
open Ast
%}

%token <int> INT
%token PLUS MINUS TIMES DIV 
%token LPAREN RPAREN
%token EOL
%left PLUS MINUS
%left TIMES DIV
%nonassoc UMINUS
%start main
%type <expr> main
%%

main:
    expr EOL { $1 }
;
expr:
  INT { Num($1) }
  | LPAREN expr RPAREN { $2 }
  | expr PLUS expr { BinOp($1, Add, $3) } 
  | expr MINUS expr { BinOp($1, Sub, $3) } 
  | expr TIMES expr { BinOp($1, Mul, $3) } 
  | expr DIV expr { BinOp($1, Div, $3) }
  | MINUS expr %prec UMINUS { Neg $2 }
;

