use std::fs;

use crate::program::*;

mod parser;
mod program;
mod tests;

fn main() {
    let contents = Box::leak(
        fs::read_to_string("./source.lang")
            .expect("File does not exist or cannot be read for some other reason")
            .into_boxed_str(),
    );

    println!("{:?}", program().parse(contents));
}
