extern crate rand;
use rand::{thread_rng, Rng};
use std::env;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789)(*&^%$#@!~";
const DEFAULT_LENGTH: usize = 12;

fn generate_password(length: usize) -> String {
    let mut rng = thread_rng();

    let password: String = (0..length) // length分だけ要素を生成してmapに回す
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        }).collect();
    
    password
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = match args.get(1) {
        Some(l) => match l.parse::<usize>() { 
            Ok(parsed_length) => parsed_length,
            Err(_) => DEFAULT_LENGTH, // usizeにパースできないケース（数値ではない等）はデフォルト値を使用
        },
        None => DEFAULT_LENGTH, // 引数がなければデフォルト値を使用
    };

    println!("Generated Password: {}", generate_password(length));
}
