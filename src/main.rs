use std::fs;

use crate::program::*;

mod parser;
mod program;
mod tests;

fn main() {
    let contents = fs::read_to_string("./source.lang")
        .expect("File does not exist or cannot be read for some other reason");
    println!("{:?}", program().parse(contents.as_str()));
}
