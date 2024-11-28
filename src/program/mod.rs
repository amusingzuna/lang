pub mod ast;
pub mod tokens;

use prelude::*;

pub mod types {
    use super::prelude::*;

    pub fn primitive() -> Parser<'static, Type> {
        identifier().map(|c| Type::Primitive(c))
    }

    pub fn types() -> Parser<'static, Type> {
        primitive()
    }
}

pub mod literal {
    use super::prelude::*;

    pub fn float_literal() -> Parser<'static, Literal> {
        float().map(|x| Literal::Float(x))
    }

    pub fn integer_literal() -> Parser<'static, Literal> {
        integer().map(|x| Literal::Integer(x))
    }

    pub fn literal() -> Parser<'static, Literal> {
        float_literal().or(integer_literal())
    }
}

pub mod expression {
    use super::prelude::*;

    pub fn expression() -> Parser<'static, Expression> {
        literal().map(|x| Expression::Literal(x))
    }
}

pub mod statement {
    use super::prelude::*;

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

    pub fn instantiate() -> Parser<'static, Statement> {
        types()
            .and(identifier())
            .left(equals())
            .and(expression())
            .map(|((a, b), c)| Statement::Instantiate(a, b, c))
    }

    pub fn no_op() -> Parser<'static, Statement> {
        Parser::pure(Statement::NoOp)
    }

    pub fn statement() -> Parser<'static, Statement> {
        strip(instantiate().or(assignment()).or(declare()).or(no_op()))
    }
}

pub mod prelude {
    pub use super::ast::*;
    pub use super::tokens::*;
    pub use super::{expression::expression, literal::literal, statement::statement, types::types};
    pub use crate::parser::*;
}

pub fn program() -> Parser<'static, Program> {
    statement()
        .left(semicolon())
        .many()
        .map(|statements| Program(statements))
}
