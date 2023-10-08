mod password_generator;
mod arg_parser;

use std::env;
use crate::password_generator::password_generator::new_password_generator;
use crate::arg_parser::parser::Parser;

fn main() {
    let args = env::args().collect();

    let perser = Parser::new();
    let config = perser.parse(args);

    // configによって出し分ける
    let generator = new_password_generator(config.algorithm);

    println!("Generated Password: {}", generator.generate_password(config.length));
}
