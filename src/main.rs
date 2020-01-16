#[macro_use]
extern crate lalrpop_util;
use std::env;
mod error;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    parser::parse_file(filename);
}
