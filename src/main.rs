mod password_generator;
mod arg_parser;

use std::env;
use crate::password_generator::{simple_password_generator::SimplePasswordGenerator, traits::PasswordGenerator};
use crate::arg_parser::parse;

fn main() {
    let args = env::args().collect();
    let parsed_length = parse(args);
    let generator = SimplePasswordGenerator::new(); // TODO: PasswordGeneratorのトレイトを作っているので、Simple以外にも対応する

    println!("Generated Password: {}", generator.generate_password(parsed_length));
}
