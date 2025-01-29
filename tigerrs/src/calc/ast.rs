
#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Num(i32),
    Neg(Box<Expr>),
}


#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}


pub fn num(n: i32) -> Expr {
    Expr::Num(n)
}
pub fn binop(l: Expr, op: BinOp, r: Expr) -> Expr {
    Expr::BinOp(Box::new(l), op, Box::new(r))
}
pub fn neg(e: Expr) -> Expr {
    Expr::Neg(Box::new(e))
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Num(n) => n.to_string(),
            Expr::BinOp(l, op, r) => format!("({} {} {})", l.to_string(), op.to_string(), r.to_string()),
            Expr::Neg(e) => format!("(-{})", e.to_string()),
        }
    }
}

impl BinOp {
    pub fn to_string(&self) -> String {
        match self {
            BinOp::Add => "+".to_string(),
            BinOp::Sub => "-".to_string(),
            BinOp::Mul => "*".to_string(),
            BinOp::Div => "/".to_string(),

        }
    }
}   

