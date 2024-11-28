use crate::ast::*;
use crate::parser::*;

pub fn semicolon() -> Parser<'static, String> {
    symbol(";")
}

pub fn primitive() -> Parser<'static, Type> {
    identifier().map(|c| Type::Primitive(c))
}

pub fn types() -> Parser<'static, Type> {
    primitive()
}

pub fn declare() -> Parser<'static, Statement> {
    types()
        .and(identifier())
        .map(|(a, b)| Statement::Declare(a, b))
}

pub fn no_op() -> Parser<'static, Statement> {
    Parser::pure(Statement::NoOp)
}

pub fn statement() -> Parser<'static, Statement> {
    strip(declare().or(no_op()))
}

pub fn program() -> Parser<'static, Program> {
    statement()
        .left(semicolon())
        .many()
        .map(|statements| Program(statements))
}
