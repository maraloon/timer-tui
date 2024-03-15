use chrono::{DateTime, Duration, Local, Utc};

pub struct Timer {
    pub start: DateTime<Local>,
    pub finish: DateTime<Local>,
}

impl Timer {
    pub fn new(timer_duration: i64) -> Timer {
        let now = Local::now();
        Timer {
            start: now,
            finish: now + Duration::milliseconds(timer_duration),
        }
    }

    pub fn ratio(&mut self) -> f64 {
        self.passed_ms() as f64 / self.duration_ms() as f64
    }

    pub fn remain_ms(&mut self) -> i64 {
        self.finish.timestamp_millis() - Utc::now().timestamp_millis()
    }

    fn passed_ms(&mut self) -> i64 {
        self.duration_ms() - self.remain_ms()
    }

    fn duration_ms(&mut self) -> i64 {
        self.finish.timestamp_millis() - self.start.timestamp_millis()
    }
}
