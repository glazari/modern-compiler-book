
type binop =
  | Add
  | Sub
  | Mul
  | Div

type expr =
  | Num of int
  | BinOp of expr * binop * expr 
  | Neg of expr


let op_to_string (op : binop) : string =
  match op with
  | Add -> "+"
  | Sub -> "-"
  | Mul -> "*"
  | Div -> "/"

let rec to_string (e : expr) : string =
  match e with
  | Num n -> string_of_int n
  | BinOp (e1, op, e2) ->
      "(" ^ (to_string e1) ^ " " ^ (op_to_string op) ^ " " ^ (to_string e2) ^ ")"
  | Neg e1 -> "(-" ^ (to_string e1) ^ ")"
