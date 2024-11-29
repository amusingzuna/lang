use crate::parser::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Let,
    Equals,
    Colon,
    Semicolon,
}

pub fn r#let() -> Parser<'static, Token> {
    symbol("let").right(Parser::pure(Token::Let))
}

pub fn equals() -> Parser<'static, Token> {
    symbol("=").right(Parser::pure(Token::Equals))
}

pub fn colon() -> Parser<'static, Token> {
    symbol(":").right(Parser::pure(Token::Colon))
}

pub fn semicolon() -> Parser<'static, Token> {
    symbol(";").right(Parser::pure(Token::Semicolon))
}
