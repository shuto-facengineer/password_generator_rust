mod generator;
mod arg_parser;

use std::env;
use crate::generator::generate_password;
use crate::arg_parser::parse;


fn main() {
    let args = env::args().collect();
    let parsed_length = parse(args);

    println!("Generated Password: {}", generate_password(parsed_length));
}
