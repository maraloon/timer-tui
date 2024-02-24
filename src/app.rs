use std::time::{Duration, Instant};
use chrono::{DateTime, Timelike, Utc};

pub struct App {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    // timer: i64,
    // remained: i64,
    // finish_time: date,
}

impl App {

    pub fn new() -> App {
        let test_time = 10; // 10 sec
        let from = Utc::now();
        App {
            from,
            to: Utc::now() + Duration::from_secs(test_time),
        }
    }

    pub fn remain(&mut self) -> i64 {
        let to_timestamp = self.to.timestamp();
        let now_timestamp = Utc::now().timestamp();
        to_timestamp - now_timestamp
    }
}
