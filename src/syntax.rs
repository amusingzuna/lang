use crate::parser::*;

pub fn dot() -> Parser<'static, String> {
    symbol(".")
}

// This parses to a string because it shouldn't explicitly save integers to a Rusty data format,
// incase someone wants numeric literals to parse to some other specific data format
pub fn integer() -> Parser<'static, String> {
    strip(digit().some().map(|c| c.into_iter().collect()))
}

pub fn float() -> Parser<'static, String> {
    strip(
        digit()
            .some()
            .qualify()
            .and(dot())
            .and(digit().many().qualify())
            .left(symbol("f").many())
            .map(|((a, b), c)| {
                let mut result = String::new();
                result.push_str(a.as_str());
                result.push_str(b.as_str());
                result.push_str(c.as_str());
                result
            }),
    )
}
