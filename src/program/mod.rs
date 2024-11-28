use crate::parser::*;

use ast::*;
use tokens::*;

pub mod ast;
pub mod tokens;

pub fn primitive() -> Parser<'static, Type> {
    identifier().map(|c| Type::Primitive(c))
}

pub fn types() -> Parser<'static, Type> {
    primitive()
}

pub fn integer_literal() -> Parser<'static, Expression> {
    integer().map(|x| Expression::Literal(x))
}

pub fn expression() -> Parser<'static, Expression> {
    integer_literal()
}

pub fn declare() -> Parser<'static, Statement> {
    types()
        .and(identifier())
        .map(|(a, b)| Statement::Declare(a, b))
}

pub fn assignment() -> Parser<'static, Statement> {
    identifier()
        .left(equals())
        .and(expression())
        .map(|(name, expr)| Statement::Assignment(name, expr))
}

pub fn no_op() -> Parser<'static, Statement> {
    Parser::pure(Statement::NoOp)
}

pub fn statement() -> Parser<'static, Statement> {
    strip(declare().or(assignment()).or(no_op()))
}

pub fn program() -> Parser<'static, Program> {
    statement()
        .left(semicolon())
        .many()
        .map(|statements| Program(statements))
}
