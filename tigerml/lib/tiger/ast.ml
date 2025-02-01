type punctuation =
  | Comma
  | Colon
  | SemiColon
  | LParen
  | RParen
  | LBrace
  | RBrace
  | LBrack
  | RBrack
  | Period
  | Minus
  | Plus
  | Times
  | Div
  | Eq
  | Less
  | Greater
  | And
  | Or

let punc_of_string (s : string) : punctuation =
  match s with
  | "," -> Comma
  | ":" -> Colon
  | ";" -> SemiColon
  | "(" -> LParen
  | ")" -> RParen
  | "{" -> LBrace
  | "}" -> RBrace
  | "[" -> LBrack
  | "]" -> RBrack
  | "." -> Period
  | "-" -> Minus
  | "+" -> Plus
  | "*" -> Times
  | "/" -> Div
  | "=" -> Eq
  | "<" -> Less
  | ">" -> Greater
  | "&" -> And
  | "|" -> Or
  | c -> failwith ("Unknown punctuation: " ^ c)

let string_of_punc (p : punctuation) : string =
  match p with
  | Comma -> ","
  | Colon -> ":"
  | SemiColon -> ";"
  | LParen -> "("
  | RParen -> ")"
  | LBrace -> "{"
  | RBrace -> "}"
  | LBrack -> "["
  | RBrack -> "]"
  | Period -> "."
  | Minus -> "-"
  | Plus -> "+"
  | Times -> "*"
  | Div -> "/"
  | Eq -> "="
  | Less -> "<"
  | Greater -> ">"
  | And -> "&"
  | Or -> "|"


type keyword =
  | While
  | For
  | To
  | Break
  | Let
  | In
  | End
  | Function
  | Var
  | Type
  | Array
  | If
  | Then
  | Else
  | Do
  | Of
  | Nil

let keyword_of_string (s : string) :  keyword option =
  match s with
  | "while" -> Some While
  | "for" -> Some For
  | "to" -> Some To
  | "break" -> Some Break
  | "let" -> Some Let
  | "in" -> Some In
  | "end" -> Some End
  | "function" -> Some Function
  | "var" -> Some Var
  | "type" -> Some Type
  | "array" -> Some Array
  | "if" -> Some If
  | "then" -> Some Then
  | "else" -> Some Else
  | "do" -> Some Do
  | "of" -> Some Of
  | "nil" -> Some Nil
  | _ -> None

let string_of_keyword (k : keyword) : string =
  match k with
  | While -> "while"
  | For -> "for"
  | To -> "to"
  | Break -> "break"
  | Let -> "let"
  | In -> "in"
  | End -> "end"
  | Function -> "function"
  | Var -> "var"
  | Type -> "type"
  | Array -> "array"
  | If -> "if"
  | Then -> "then"
  | Else -> "else"
  | Do -> "do"
  | Of -> "of"
  | Nil -> "nil"


type expr = 
  Iden of string
  | Num of int
  | List of expr list
  | Punct of punctuation
  | Keyword of keyword
  | StrLit of string


let rec to_string (e : expr) : string =
  match e with
  | Iden s -> "Iden(" ^ s ^ ")"
  | Num n -> "Num(" ^ string_of_int n ^ ")"
  | Punct p -> "Punt(" ^ string_of_punc p ^ ")"
  | Keyword k -> "Keyword(" ^ string_of_keyword k ^ ")"
  | StrLit s -> "\"" ^ s ^ "\""
  | List [] -> "]"
  | List (e :: rest) ->  to_string e ^ ", "  ^ to_string (List rest)





