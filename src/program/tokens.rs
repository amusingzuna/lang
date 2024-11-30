use super::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Let,
    True,
    False,
    Equals,
    Colon,
    Semicolon,
}

pub fn let_key<'a>() -> Parser<'a, Token> {
    symbol("let").right(Parser::pure(Token::Let))
}

pub fn true_key<'a>() -> Parser<'a, Token> {
    symbol("true").right(Parser::pure(Token::True))
}

pub fn false_key<'a>() -> Parser<'a, Token> {
    symbol("false").right(Parser::pure(Token::False))
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
