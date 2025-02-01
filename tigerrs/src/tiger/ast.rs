#[derive(Debug)]
pub enum Expr {
    Iden(String),
    Num(i64),
    Punct(Punctuation),
    Keyword(Keyword),
    StrLit(String),
    List(Vec<Expr>),
}


impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Iden(s) => "id(".to_string() + s + ")",
            Expr::Num(n) => "num(".to_string() +  &n.to_string() + ")",
            Expr::Punct(p) => "punct(".to_string() + &p.to_string() + ")",
            Expr::Keyword(k) => "keyword(".to_string() + &k.to_string() + ")",
            Expr::StrLit(s) => "\"".to_string() + s + "\"",
            Expr::List(l) => l.iter().fold("list(".to_string(), |a, b| a + &b.to_string() + ", ") + ")",
        }
    }
}



#[derive(Debug)]
pub enum Punctuation {
  Comma,
  Colon,
  SemiColon,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBrack,
  RBrack,
  Period,
  Minus,
  Plus,
  Times,
  Div,
  Eq,
  Less,
  Greater,
  And,
  Or,
}

impl Punctuation {
    pub fn to_string(&self) -> String {
        match self {
            Punctuation::Comma => ",".to_string(),
            Punctuation::Colon => ":".to_string(),
            Punctuation::SemiColon => ";".to_string(),
            Punctuation::LParen => "(".to_string(),
            Punctuation::RParen => ")".to_string(),
            Punctuation::LBrace => "{".to_string(),
            Punctuation::RBrace => "}".to_string(),
            Punctuation::LBrack => "[".to_string(),
            Punctuation::RBrack => "]".to_string(),
            Punctuation::Period => ".".to_string(),
            Punctuation::Minus => "-".to_string(),
            Punctuation::Plus => "+".to_string(),
            Punctuation::Times => "*".to_string(),
            Punctuation::Div => "/".to_string(),
            Punctuation::Eq => "=".to_string(),
            Punctuation::Less => "<".to_string(),
            Punctuation::Greater => ">".to_string(),
            Punctuation::And => "&".to_string(),
            Punctuation::Or => "|".to_string(),
        }
    }
    pub fn of_string(s: &str) -> Option<Punctuation> {
        match s {
            "," => Some(Punctuation::Comma),
            ":" => Some(Punctuation::Colon),
            ";" => Some(Punctuation::SemiColon),
            "(" => Some(Punctuation::LParen),
            ")" => Some(Punctuation::RParen),
            "{" => Some(Punctuation::LBrace),
            "}" => Some(Punctuation::RBrace),
            "[" => Some(Punctuation::LBrack),
            "]" => Some(Punctuation::RBrack),
            "." => Some(Punctuation::Period),
            "-" => Some(Punctuation::Minus),
            "+" => Some(Punctuation::Plus),
            "*" => Some(Punctuation::Times),
            "/" => Some(Punctuation::Div),
            "=" => Some(Punctuation::Eq),
            "<" => Some(Punctuation::Less),
            ">" => Some(Punctuation::Greater),
            "&" => Some(Punctuation::And),
            "|" => Some(Punctuation::Or),
            _ => None,
        }
    }
}


#[derive(Debug)]
pub enum Keyword {
  While,
  For,
  To,
  Break,
  Let,
  In,
  End,
  Function,
  Var,
  Type,
  Array,
  If,
  Then,
  Else,
  Do,
  Of,
  Nil,
}

impl Keyword {
    pub fn to_string(&self) -> String {
        match self {
            Keyword::While => "while".to_string(),
            Keyword::For => "for".to_string(),
            Keyword::To => "to".to_string(),
            Keyword::Break => "break".to_string(),
            Keyword::Let => "let".to_string(),
            Keyword::In => "in".to_string(),
            Keyword::End => "end".to_string(),
            Keyword::Function => "function".to_string(),
            Keyword::Var => "var".to_string(),
            Keyword::Type => "type".to_string(),
            Keyword::Array => "array".to_string(),
            Keyword::If => "if".to_string(),
            Keyword::Then => "then".to_string(),
            Keyword::Else => "else".to_string(),
            Keyword::Do => "do".to_string(),
            Keyword::Of => "of".to_string(),
            Keyword::Nil => "nil".to_string(),
        }
    }
    pub fn of_string(s: &str) -> Option<Keyword> {
        match s {
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "to" => Some(Keyword::To),
            "break" => Some(Keyword::Break),
            "let" => Some(Keyword::Let),
            "in" => Some(Keyword::In),
            "end" => Some(Keyword::End),
            "function" => Some(Keyword::Function),
            "var" => Some(Keyword::Var),
            "type" => Some(Keyword::Type),
            "array" => Some(Keyword::Array),
            "if" => Some(Keyword::If),
            "then" => Some(Keyword::Then),
            "else" => Some(Keyword::Else),
            "do" => Some(Keyword::Do),
            "of" => Some(Keyword::Of),
            "nil" => Some(Keyword::Nil),
            _ => None,
        }
    }
}
