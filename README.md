# Modern Compiler Implmentation Book

This repo is to follow the "Modern compiler implementation in ML" book, by Andrew Appel.

I am implmenting the examples both in Ocaml and Rust. Ocaml was chosen to match the original 
implementation as much as possible, while Rust is my prefered lanauge. The reason not to do
it directly in Rust was to make sure I wouldn't get stuck with ownership issues if some of the 
datastructures used actually rely on multiple ownerships.


# 1. Intro (Straight-line programing language)

Grammar:
```ebnf
Stm ::= CompoundStm 
    | AssignStm 
    | PrintStm

CompoundStm ::= Stm ";" Stm
AsignStm ::= id ":=" Exp
PrintStm ::= "print" "(" Exp {"," Exp} ")"

Exp ::= IdExp 
    | NumExp 
    | OpExp 
    | EseqExp


IdExp ::= id
NumExp ::= num
OpExp ::= Exp Binop Exp
Binop ::= "+" | "-" | "*" | "/"
EseqExp ::= "(" Stm "," Exp ")"
```


Example program:
```
a := 5 + 3;
b := (print(a, a-1), 10*a);
print(b)
```

Output:
```
8 7
80
```
