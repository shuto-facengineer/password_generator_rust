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