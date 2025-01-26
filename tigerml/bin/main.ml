let () = Printf.printf "Hello, %s!\n" Tigerml.Test1.v

let () =
  Printf.printf "Running straight_line.ml\n";
  Tigerml.Straight_line.main ();
  Printf.printf "Running calc.ml\n";
  Tigerml.Calc.main ();
