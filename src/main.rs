extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::env;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    parser::parse_files(filename);
}
