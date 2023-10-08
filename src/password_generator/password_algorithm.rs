#[derive(Debug, PartialEq)]
pub enum PasswordAlgorithm {
    Simple,
    Numeric,
    // 新しいPasswordGeneratorを追加したらこちらにも追加すること
}

impl PasswordAlgorithm {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "simple" => PasswordAlgorithm::Simple,
            "numeric" => PasswordAlgorithm::Numeric,
            // 新しいPasswordGeneratorを追加したらこちらにも追加すること
            _ => {
                 // 該当がなくてもエラーは出さずにデフォルト値で対応する
                eprintln!("Error: invalid password algorithm, using default simple algorithm ", );
                PasswordAlgorithm::Simple
            },
        }
    }
}
