use std::i64;

use regex::{self, Regex};

// pub fn parse_time_argument(time_string: String) -> i64 {
pub fn parse_time_argument(time_string: String) -> Result<i64, String> {
    let re = Regex::new(r"(\d+)([^0-9]+)").unwrap();
    let error: &str = "time argument must be NUMBER[s|m|h] format";

    if let Some(captures) = re.captures(&time_string) {
        let value: i64 = captures[1].parse().unwrap();
        match &captures[2] {
            "s" => Ok(value * 1000),
            "m" => Ok(value * 1000 * 60),
            "h" => Ok(value * 1000 * 60 * 60),
            // _ => panic!("{}", error),
            _ => Err(error.to_string()),
        }
    } else {
        let result: Result<i64, _> = time_string.parse();
        match result {
            Ok(parsed_number) => Ok(parsed_number * 1000),
            Err(_) => {
                Err(error.to_string())
            }
        }
    }
}
