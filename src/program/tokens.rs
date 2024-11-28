use crate::parser::*;

#[derive(Clone)]
pub enum Token {
    Equals,
    Semicolon,
}

pub fn equals() -> Parser<'static, Token> {
    symbol("=").right(Parser::pure(Token::Equals))
}

pub fn semicolon() -> Parser<'static, Token> {
    symbol(";").right(Parser::pure(Token::Semicolon))
}
