use crate::ast::*;
use crate::parser::*;

pub fn semicolon() -> Parser<'static, String> {
    symbol(";")
}

pub fn no_op() -> Parser<'static, Statement> {
    Parser::pure(Statement::NoOp)
}

pub fn statement() -> Parser<'static, Statement> {
    strip(no_op())
}

pub fn program() -> Parser<'static, Program> {
    statement()
        .left(semicolon())
        .many()
        .map(|statements| Program(statements))
}
