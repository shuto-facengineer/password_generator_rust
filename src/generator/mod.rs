use rand::{thread_rng, Rng};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789)(*&^%$#@!~";


pub fn generate_password(length: usize) -> String {
    let mut rng = thread_rng();

    let password: String = (0..length) // length分だけ要素を生成してmapに回す
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        }).collect();
    
    password
}