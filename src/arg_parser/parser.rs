use crate::arg_parser::consts::{DEFAULT_LENGTH, DEFAULT_ALGORITHM};
use crate::password_generator::password_algorithm::PasswordAlgorithm;
use crate::arg_parser::config::PasswordConfig;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, args: Vec<String>) -> PasswordConfig {
        // 第一引数はパスワード長
        let length = match args.get(1) {
            Some(s) => match s.parse::<usize>() { 
                Ok(length) => length,
                Err(_) => {
                    // usizeにパースできないケース（数値ではない等）はデフォルト値を使用
                    eprintln!("Error: invalid password length, using default length 12");
                    DEFAULT_LENGTH
                }
            },
            None => DEFAULT_LENGTH, // 引数がなければデフォルト値を使用
        };

        // 第二引数はアルゴリズム
        let algorithm =  match args.get(2) {
            Some(s) => PasswordAlgorithm::from_str(s),
            None => DEFAULT_ALGORITHM,
        };
            

        PasswordConfig { length, algorithm }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_length() { // 有効なパスワード長
        let parser = Parser::new();
        let args = vec!["progname".to_string(), "15".to_string()];
        let config = parser.parse(args);
        assert_eq!(config.length, 15);
    }

    #[test]
    fn test_parse_invalid_length() { // 無効なパスワード長
        let parser = Parser::new();
        let args = vec!["progname".to_string(), "invalid".to_string()];
        let config = parser.parse(args);
        assert_eq!(config.length, DEFAULT_LENGTH);
    }

    #[test]
    fn test_parse_omitted_length() { // パスワード長を省略
        let parser = Parser::new();
        let args = vec!["progname".to_string()];
        let config = parser.parse(args);
        assert_eq!(config.length, DEFAULT_LENGTH);
    }

    #[test]
    fn test_parse_valid_algorithm_simple() { // 有効なアルゴリズム（Simple）
        let parser = Parser::new();
        let args = vec!["progname".to_string(), "12".to_string(), "simple".to_string()];
        let config = parser.parse(args);
        assert_eq!(config.algorithm, PasswordAlgorithm::Simple);
    }

    #[test]
    fn test_parse_valid_algorithm_numeric() { // 有効なアルゴリズム（Numeric）
        let parser = Parser::new();
        let args = vec!["progname".to_string(), "12".to_string(), "numeric".to_string()];
        let config = parser.parse(args);
        assert_eq!(config.algorithm, PasswordAlgorithm::Numeric);
    }

    #[test]
    fn test_parse_invalid_algorithm() { // 無効なアルゴリズム
        let parser = Parser::new();
        let args = vec!["progname".to_string(), "12".to_string(), "invalid".to_string()];
        let config = parser.parse(args);
        assert_eq!(config.algorithm, DEFAULT_ALGORITHM);
    }

    #[test]
    fn test_parse_omitted_algorithm() { // アルゴリズムを省略
        let parser = Parser::new();
        let args = vec!["progname".to_string(), "12".to_string()];
        let config = parser.parse(args);
        assert_eq!(config.algorithm, DEFAULT_ALGORITHM);
    }
}
