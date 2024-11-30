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
