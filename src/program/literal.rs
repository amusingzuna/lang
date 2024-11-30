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

pub fn array_literal<'a>() -> Parser<'a, Literal> {
    let static_parser = set(expression().left(semicolon()).and(expression()))
        .map(|(x, y)| Literal::Array(Array::Static(Box::new(x), Box::new(y))));
    let dynamic_parser =
        set(delimited(expression(), comma())).map(|x| Literal::Array(Array::Dynamic(x)));

    static_parser.or(dynamic_parser)
}

pub fn reference_literal<'a>() -> Parser<'a, Literal> {
    identifier().map(|x| Literal::Reference(x))
}

pub fn literal<'a>() -> Parser<'a, Literal> {
    float_literal()
        .or(integer_literal())
        .or(bool_literal())
        .or(array_literal())
        .or(reference_literal())
}
