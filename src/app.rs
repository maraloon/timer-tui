use chrono::{DateTime, Utc};
use std::time::Duration;

pub struct App {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl App {
    pub fn new() -> App {
        let test_time = 13000;
        let from = Utc::now();
        let to = from + Duration::from_millis(test_time);

        App { from, to }
    }

    pub fn ratio(&mut self) -> f64 {
        let full: f64 = (self.to.timestamp_millis() - self.from.timestamp_millis()) as f64;
        let passed: f64 = self.passed() as f64;
        passed / full
    }

    pub fn remain(&mut self) -> i64 {
        let to_timestamp = self.to.timestamp_millis();
        let now_timestamp = Utc::now().timestamp_millis();
        to_timestamp - now_timestamp
    }

    pub fn remainSeconds(&mut self) -> i64 {
        let remain = self.remain();
        let seconds = ((remain / 1000) as f64).ceil();
        seconds as i64
    }

    pub fn passed(&mut self) -> i64 {
        let full_percent = self.to.timestamp_millis() - self.from.timestamp_millis();
        let remain = self.remain();
        full_percent - remain
    }
}
