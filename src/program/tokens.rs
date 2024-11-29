use crate::parser::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Let,
    Equals,
    Colon,
    Semicolon,
}

pub fn let_key<'a>() -> Parser<'a, Token> {
    symbol("let").right(Parser::pure(Token::Let))
}

pub fn equals<'a>() -> Parser<'a, Token> {
    symbol("=").right(Parser::pure(Token::Equals))
}

pub fn colon<'a>() -> Parser<'a, Token> {
    symbol(":").right(Parser::pure(Token::Colon))
}

pub fn semicolon<'a>() -> Parser<'a, Token> {
    symbol(";").right(Parser::pure(Token::Semicolon))
}
