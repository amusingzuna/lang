use prelude::*;

pub mod ast;
pub mod expression;
pub mod literal;
pub mod statement;
pub mod tokens;
pub mod types;

pub mod prelude {
    pub use super::{
        ast::*, expression::expression, literal::literal, statement::statement, tokens::*,
        types::types,
    };
    pub use crate::parser::*;
}

pub fn program<'a>() -> Parser<'a, Program> {
    statement()
        .left(semicolon())
        .many()
        .map(|statements| Program(statements))
}
