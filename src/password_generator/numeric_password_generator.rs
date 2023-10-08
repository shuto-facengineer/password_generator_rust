use rand::{thread_rng, Rng};
use crate::password_generator::PasswordGenerator;

pub struct NumericPasswordGenerator;

impl NumericPasswordGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordGenerator for NumericPasswordGenerator {
    fn generate_password(&self, length: usize) -> String {
        let mut rng = thread_rng();
        let password = (0..length)
            .map(|_| {
                rng.gen_range(0..10).to_string().chars().next().unwrap()
            })
        .collect();
        password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() { // length分だけ要素を生成してmapに回す
        let generator = NumericPasswordGenerator::new();
        let length = 10;
        let password = generator.generate_password(length);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn test_generate_password_charset() { // 想定通りのキャラクターセットを利用すること（数字のみになっていること）
        let generator = NumericPasswordGenerator::new();
        let length = 1000;
        let password = generator.generate_password(length);
        for ch in password.chars() {
            assert!(ch.is_numeric());
        }
    }
}
