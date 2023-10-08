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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() { // パスワードのサイズが指定した通りになること
        let length = 10;
        let password = generate_password(length);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn test_generate_password_charset() { // 想定通りのキャラクターセットを利用すること
        let length = 1000;
        let password = generate_password(length);
        for ch in password.chars() {
            assert!(CHARSET.contains(&(ch as u8)));
        }
    }
}
