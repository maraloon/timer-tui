use chrono::{DateTime, Timelike, Utc};
use std::time::{Duration, Instant};

pub struct App {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    // full_percent: i64,
    // leak_percent: i64,
    // timer: i64,
    // remained: i64,
    // finish_time: date,
}

impl App {
    pub fn new() -> App {
        let test_time = 10; // 10 sec
        let from = Utc::now();
        let to = from + Duration::from_secs(test_time);

        // let full_percent = to.timestamp() - from.timestamp();

        App {
            from,
            to,
            // full_percent,
        }
    }

    pub fn ratio(&mut self) -> f64 {
        let full: f64 = (self.to.timestamp() - self.from.timestamp()) as f64;
        let passed: f64 = self.passed() as f64;
        passed / full
    }

    pub fn remain(&mut self) -> i64 {
        let to_timestamp = self.to.timestamp();
        let now_timestamp = Utc::now().timestamp();
        to_timestamp - now_timestamp
    }

    pub fn passed(&mut self) -> i64 {
        let full_percent = self.to.timestamp() - self.from.timestamp();
        let remain = self.remain();
        full_percent - remain
    }
}
