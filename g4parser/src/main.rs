mod parser;
mod ast;

use parser::Parser;
use parse_tools::CharTools;
use std::fs;

fn main() {
    let p = Parser::new();

    let contents = fs::read_to_string("../tests/lisp.g4")
        .expect("Failed to read tests/lisp.g4");

    let res = p.parse(contents);

    match res {
        Ok(ast) => {
            println!("Parsed successfully: {:#?}", ast);
        },
        Err(e) => {
            println!("Error parsing: {}", e);
        }
    }
    println!("Hello, world!");
}
