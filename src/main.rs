use crate::program::*;

mod parser;
mod program;
mod tests;

fn main() {
    let program_parser = program();
    let ast = program_parser.parse("a = {a = 12};");
    println!("{:?}", ast);
}
