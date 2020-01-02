#[macro_use]
extern crate lalrpop_util;
use std::env;
mod error;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    match parser::parse_file(filename) {
        Ok(expr) => {
            dbg!(expr);
        }
        Err(e) => {
            println!("Error parsing file: {:?}", e);
        }
    };
}
