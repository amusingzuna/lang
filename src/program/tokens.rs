use crate::parser::*;

pub fn semicolon() -> Parser<'static, String> {
    symbol(";")
}
