use super::prelude::*;

pub fn instantiate<'a>() -> Parser<'a, Statement> {
    let_key()
        .right(identifier())
        .and(otherwise(colon().right(types()).map(|x| Some(x)), None))
        .left(equals())
        .and(expression())
        .map(|((a, b), c)| Statement::Instantiate(b, a, c))
}

pub fn assign<'a>() -> Parser<'a, Statement> {
    identifier()
        .left(equals())
        .and(expression())
        .map(|(name, expr)| Statement::Assign(name, expr))
}

pub fn declare<'a>() -> Parser<'a, Statement> {
    let_key()
        .right(identifier())
        .and(otherwise(colon().right(types()).map(|x| Some(x)), None))
        .map(|(a, b)| Statement::Declare(b, a))
}

pub fn variable<'a>() -> Parser<'a, Statement> {
    instantiate().or(assign()).or(declare())
}

pub fn expression_stat<'a>() -> Parser<'a, Statement> {
    expression().map(|x| Statement::Expression(x))
}

pub fn no_op<'a>() -> Parser<'a, Statement> {
    Parser::pure(Statement::NoOp)
}

pub fn statement<'a>() -> Parser<'a, Statement> {
    Parser::lazy(|| strip(variable().or(expression_stat()).or(no_op())))
}
