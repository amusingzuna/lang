pub mod ast;
pub mod tokens;

use prelude::*;

pub mod types {
    use super::prelude::*;

    pub fn atomic<'a>() -> Parser<'a, Type> {
        identifier().map(|c| Type::Atomic(c))
    }

    pub fn types<'a>() -> Parser<'a, Type> {
        atomic()
    }
}

pub mod literal {
    use super::prelude::*;

    pub fn float_literal<'a>() -> Parser<'a, Literal> {
        float().map(|x| Literal::Float(x))
    }

    pub fn integer_literal<'a>() -> Parser<'a, Literal> {
        integer().map(|x| Literal::Integer(x))
    }

    pub fn reference_literal<'a>() -> Parser<'a, Literal> {
        identifier().map(|x| Literal::Reference(x))
    }

    pub fn literal<'a>() -> Parser<'a, Literal> {
        float_literal()
            .or(integer_literal())
            .or(reference_literal())
    }
}

pub mod expression {
    use super::prelude::*;

    pub fn literal_expr<'a>() -> Parser<'a, Expression> {
        literal().map(|x| Expression::Literal(x))
    }

    pub fn block_expr<'a>() -> Parser<'a, Expression> {
        block(statement().left(semicolon()).many()).map(|x| Expression::Block(x))
    }

    pub fn expression<'a>() -> Parser<'a, Expression> {
        Parser::lazy(|| literal_expr().or(block_expr()))
    }
}

pub mod statement {
    use super::prelude::*;

    pub fn instantiate<'a>() -> Parser<'a, Statement> {
        let_key()
            .right(identifier())
            .and(otherwise(colon().right(types()).map(|x| Some(x)), None))
            .left(equals())
            .and(expression())
            .map(|((a, b), c)| Statement::Instantiate(b, a, c))
    }

    pub fn assignment<'a>() -> Parser<'a, Statement> {
        identifier()
            .left(equals())
            .and(expression())
            .map(|(name, expr)| Statement::Assignment(name, expr))
    }

    pub fn declare<'a>() -> Parser<'a, Statement> {
        let_key()
            .right(identifier())
            .and(otherwise(colon().right(types()).map(|x| Some(x)), None))
            .map(|(a, b)| Statement::Declare(b, a))
    }

    pub fn variable<'a>() -> Parser<'a, Statement> {
        instantiate().or(assignment()).or(declare())
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
}

pub mod prelude {
    pub use super::ast::*;
    pub use super::tokens::*;
    pub use super::{expression::expression, literal::literal, statement::statement, types::types};
    pub use crate::parser::*;
}

pub fn program<'a>() -> Parser<'a, Program> {
    statement()
        .left(semicolon())
        .many()
        .map(|statements| Program(statements))
}
