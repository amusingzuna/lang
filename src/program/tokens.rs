use crate::parser::*;

#[derive(PartialEq, Eq, Debug, Clone)]
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
