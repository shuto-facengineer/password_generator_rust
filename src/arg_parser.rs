const DEFAULT_LENGTH: usize = 12;

pub fn parse(args: Vec<String>) -> usize {
    let length = match args.get(1) {
        Some(l) => match l.parse::<usize>() { 
            Ok(parsed_length) => parsed_length,
            Err(_) => DEFAULT_LENGTH, // usizeにパースできないケース（数値ではない等）はデフォルト値を使用
        },
        None => DEFAULT_LENGTH, // 引数がなければデフォルト値を使用
    };

    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_returns_default_with_no_args() { // args指定なしの場合、デフォルト長が返ること
        let args = vec!["program_name".to_string()];
        assert_eq!(parse(args), DEFAULT_LENGTH);
    }

    #[test]
    fn parse_returns_default_with_invalid_arg() { // args指定が不正な場合、デフォルト長が返ること
        let args = vec!["program_name".to_string(), "invalid".to_string()];
        assert_eq!(parse(args), DEFAULT_LENGTH);
    }

    #[test]
    fn parse_returns_parsed_value_with_valid_arg() { // 適切なargsが指定された場合、指定された長さが返ること
        let args = vec!["program_name".to_string(), "8".to_string()];
        assert_eq!(parse(args), 8);
    }
}