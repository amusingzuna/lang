use syntax::program;

mod ast;
mod parser;
mod syntax;
mod tests;

fn main() {
    let program_parser = program();

    let ast = program_parser.parse(";;");

    println!("{:?}", ast);
}
