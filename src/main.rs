use crate::program::*;

mod parser;
mod program;
mod tests;

fn main() {
    let program_parser = program();
    let ast = program_parser.parse("a = 123;");
    println!("{:?}", ast);
}
