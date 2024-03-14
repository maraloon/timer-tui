use regex::{self, Regex};

pub fn parse_time_argument(time_string: String) -> i64 {
    let re = Regex::new(r"(\d+)([^0-9]+)").unwrap();
    let error: &str = "time argument must be NUMBER[s|m|h] format";

    if let Some(captures) = re.captures(&time_string) {
        let value: i64 = captures[1].parse().unwrap();
        match &captures[2] {
            "s" => value * 1000,
            "m" => value * 1000 * 60,
            "h" => value * 1000 * 60 * 60,
            _ => panic!("{}", error),
        }
    } else {
        let result: Result<i64, _> = time_string.parse();
        match result {
            Ok(parsed_number) => parsed_number * 1000,
            Err(_) => {
                panic!("{}", error)
            }
        }
    }
}
