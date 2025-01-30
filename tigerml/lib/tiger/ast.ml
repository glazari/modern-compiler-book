type expr = 
  Iden of string
  | Num of int
  | List of expr list


let rec to_string (e : expr) : string =
  match e with
  | Iden s -> "Iden(" ^ s ^ ")"
  | Num n -> "Num(" ^ string_of_int n ^ ")"
  | List [] -> "]"
  | List (e :: rest) ->  to_string e ^ ", "  ^ to_string (List rest)


