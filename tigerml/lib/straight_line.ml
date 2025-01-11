type id = string
type binop = Plus | Minus | Times | Div

type stm =
  | CompoundStm of stm * stm
  | AssignStm of id * exp
  | PrintStm of exp list

and exp =
  | IdExp of id
  | NumExp of int
  | OpExp of exp * binop * exp
  | EseqExp of stm * exp

let exampleProg =
  CompoundStm
    ( AssignStm ("a", OpExp (NumExp 5, Plus, NumExp 3)),
      CompoundStm
        ( AssignStm
            ( "b",
              EseqExp
                ( PrintStm [ IdExp "a"; OpExp (IdExp "a", Minus, NumExp 1) ],
                  OpExp (NumExp 10, Times, IdExp "a") ) ),
          PrintStm [ IdExp "b" ] ) )

type envir = (id * int) list

let rec lookup env x =
  match env with
  | [] -> failwith "Variable not found"
  | (y, v) :: rest -> if x = y then v else lookup rest x

let rec evalExp (exp : exp) (env : envir) : envir * int =
  match exp with
  | IdExp id -> (env, lookup env id)
  | NumExp n -> (env, n)
  | OpExp (e1, op, e2) ->
      let env', v1 = evalExp e1 env in
      let env'', v2 = evalExp e2 env' in
      let v =
        match op with
        | Plus -> v1 + v2
        | Minus -> v1 - v2
        | Times -> v1 * v2
        | Div -> v1 / v2
      in
      (env'', v)
  | EseqExp (s, e) ->
      let env', _ = evalStm s env in
      evalExp e env'

and printExpVal (exp : exp) (env : envir) : unit =
  let _, v = evalExp exp env in
  print_int v;
  print_string " "

and evalStm (stm : stm) (env : envir) : envir * int =
  match stm with
  | CompoundStm (s1, s2) ->
      let env', _ = evalStm s1 env in
      evalStm s2 env'
  | AssignStm (id, exp) ->
      let env', v = evalExp exp env in
      ((id, v) :: env', v)
  | PrintStm exps ->
      List.iter (fun e -> printExpVal e env) exps;
      print_newline ();
      (env, 0)

let envToStr (env : envir) : string =
  List.fold_left
    (fun acc (id, v) -> acc ^ id ^ " = " ^ string_of_int v ^ ", ")
    "" env

let interp (prog : stm) : unit =
  let env, v = evalStm prog [] in
  Printf.printf "Program executed successfully\n";
  Printf.printf "Final environment: %s\n" (envToStr env);
  Printf.printf "Final value: %d\n" v

let main () = interp exampleProg
