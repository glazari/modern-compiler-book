#[derive(Debug)]
pub enum Expr {
    Iden(String),
}


impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Iden(s) => "id(".to_string() + s + ")",
        }
    }
}
