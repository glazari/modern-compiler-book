type Id = String;
type Env = Vec<(Id, i32)>;

macro_rules! printExp {
    ($($x:expr),*) => {
        Stm::Print(vec![$($x),*])
    };
}

fn lookup(id: &Id, env: &Env) -> i32 {
    env.iter()
        .rev()
        .find(|(x, _)| x == id)
        .map(|(_, v)| *v)
        .unwrap()
}

fn concat(env: &Env, id: Id, val: i32) -> Env {
    let mut new_env = env.clone();
    new_env.push((id, val));
    new_env
}

enum BinOp {
    Plus,
    Minus,
    Times,
    Div,
}

enum Stm {
    Compound(Box<Stm>, Box<Stm>),
    Assign(Id, Exp),
    Print(Vec<Exp>),
}

enum Exp {
    Id(Id),
    Num(i32),
    Op(Box<Exp>, BinOp, Box<Exp>),
    Eseq(Box<Stm>, Box<Exp>),
}

fn exampl_prog() -> Stm {
    compound(
        assign("a", op(Exp::Num(5), BinOp::Plus, Exp::Num(3))),
        compound(
            assign(
                "b",
                eseq(
                    printExp!(id("a"), op(id("a"), BinOp::Minus, Exp::Num(1))),
                    op(Exp::Num(10), BinOp::Times, id("a")),
                ),
            ),
            Stm::Print(vec![id("b")]),
        ),
    )
}

fn compound(s1: Stm, s2: Stm) -> Stm {
    Stm::Compound(Box::new(s1), Box::new(s2))
}
fn assign(id: &str, exp: Exp) -> Stm {
    Stm::Assign(id.to_string(), exp)
}
fn op(left: Exp, op: BinOp, right: Exp) -> Exp {
    Exp::Op(Box::new(left), op, Box::new(right))
}
fn id(id: &str) -> Exp {
    Exp::Id(id.to_string())
}
fn eseq(stm: Stm, exp: Exp) -> Exp {
    Exp::Eseq(Box::new(stm), Box::new(exp))
}

fn eval_exp(exp: &Exp, env: Env) -> (Env, i32) {
    match exp {
        Exp::Id(id) => {
            let v = lookup(id, &env);
            (env, v)
        }
        Exp::Num(num) => (env, *num),
        Exp::Op(left, op, right) => {
            let (env, l) = eval_exp(left, env);
            let (env, r) = eval_exp(right, env);
            let v = match op {
                BinOp::Plus => l + r,
                BinOp::Minus => l - r,
                BinOp::Times => l * r,
                BinOp::Div => l / r,
            };
            (env, v)
        }
        Exp::Eseq(stm, exp) => {
            let (env, _) = eval_stm(stm, env);
            eval_exp(exp, env)
        }
    }
}

fn eval_stm(stm: &Stm, env: Env) -> (Env, i32) {
    match stm {
        Stm::Compound(s1, s2) => {
            let (env, _) = eval_stm(s1, env);
            eval_stm(s2, env)
        }
        Stm::Assign(id, exp) => {
            let (env, v) = eval_exp(exp, env);
            (concat(&env, id.clone(), v), v)
        }
        Stm::Print(exps) => {
            let (env, v) = exps.iter().fold((env, 0), |(env, _), exp| {
                let (env, v) = eval_exp(exp, env);
                print!("{} ", v);
                (env, 0)
            });
            println!();
            (env, v)
        }
    }
}

fn env_to_string(env: &Env) -> String {
    env.iter()
        .rev()
        .map(|(id, v)| format!("{} = {}", id, v))
        .collect::<Vec<String>>()
        .join(", ")
}

fn interp(stm: Stm) {
    let (env, v) = eval_stm(&stm, vec![]);
    println!("Program executed successfully");
    println!("Final environment: {}", env_to_string(&env));
    println!("Return value: {}", v);
}

pub fn main() {
    interp(exampl_prog());
}
