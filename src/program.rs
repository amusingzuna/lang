use prelude::*;

pub mod tokens {
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
}

pub mod ast {
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum Type {
        Atomic(String),
        Array(Box<Type>),
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum Literal {
        Float(String),
        Integer(String),
        Boolean(bool),
        Reference(String),
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum Expression {
        Block(Vec<Statement>),
        Literal(Literal),
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum Statement {
        Declare(Option<Type>, String),
        Assignment(String, Expression),
        Instantiate(Option<Type>, String, Expression),
        Expression(Expression),
        NoOp,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Program(pub Vec<Statement>);
}

pub mod types {
    use super::prelude::*;

    pub fn atomic<'a>() -> Parser<'a, Type> {
        identifier().map(|c| Type::Atomic(c))
    }

    pub fn array<'a>() -> Parser<'a, Type> {
        set(types()).map(|c| Type::Array(Box::new(c)))
    }

    pub fn types<'a>() -> Parser<'a, Type> {
        Parser::lazy(|| array().or(atomic()))
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

    pub fn bool_literal<'a>() -> Parser<'a, Literal> {
        true_key().or(false_key()).map(|x| {
            Literal::Boolean(match x {
                Token::True => true,
                Token::False => false,
                _ => panic!("Nah bruh what how you do this"),
            })
        })
    }

    pub fn reference_literal<'a>() -> Parser<'a, Literal> {
        identifier().map(|x| Literal::Reference(x))
    }

    pub fn literal<'a>() -> Parser<'a, Literal> {
        float_literal()
            .or(integer_literal())
            .or(bool_literal())
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
