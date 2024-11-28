use crate::parser::*;

// This parses to a string because it shouldn't explicitly save integers to a Rusty data format,
// incase someone wants numeric literals to parse to some other specific data format
pub fn integer() -> Parser<'static, String> {
    strip(digit().some().map(|c| c.into_iter().collect()))
}

pub fn float() -> Parser<'static, String> {
    Parser::pure("".to_string())
}
