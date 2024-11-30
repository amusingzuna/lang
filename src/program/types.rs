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
