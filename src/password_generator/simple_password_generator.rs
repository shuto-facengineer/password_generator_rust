use rand::{thread_rng, Rng};
use crate::password_generator::PasswordGenerator;
use crate::password_generator::consts::CHARSET;

pub struct SimplePasswordGenerator;

impl SimplePasswordGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordGenerator for SimplePasswordGenerator {
    fn generate_password(&self, length: usize) -> String {
        let mut rng = thread_rng();

        let password: String = (0..length) // length分だけ要素を生成してmapに回す
            .map(|_| {
              let idx = rng.gen_range(0..CHARSET.len());
              CHARSET[idx] as char
            }).collect();
    
        password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() { // パスワードのサイズが指定した通りになること
        let generator = SimplePasswordGenerator::new();
        let length = 10;
        let password = generator.generate_password(length);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn test_generate_password_charset() { // 想定通りのキャラクターセットを利用すること
        let generator = SimplePasswordGenerator::new();
        let length = 1000;
        let password = generator.generate_password(length);
        for ch in password.chars() {
            assert!(CHARSET.contains(&(ch as u8)));
        }
    }
}
