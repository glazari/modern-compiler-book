use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser2);

// Re-export the parser
pub use parser2::ExprParser; 
