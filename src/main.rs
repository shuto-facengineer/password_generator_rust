use std::env;
mod generator;
mod arg_parser;

fn main() {
    let args = env::args().collect();
    let parsed_length = arg_parser::parse(args);

    println!("Generated Password: {}", generator::generate_password(parsed_length));
}
