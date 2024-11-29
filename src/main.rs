use crate::program::*;

mod parser;
mod program;
mod tests;

fn main() {
    let program_parser = program();
    let ast = program_parser.parse("let a = a213;");
    println!("{:?}", ast);
}
