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
