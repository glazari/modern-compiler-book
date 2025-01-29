use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

// Re-export the parser
pub use parser::ExprParser; 
