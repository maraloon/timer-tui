use chrono::{ DateTime, Duration, Local, Utc};

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
        let full: f64 = (self.finish.timestamp_millis() - self.start.timestamp_millis()) as f64;
        let passed: f64 = self.passed_ms() as f64;
        passed / full
    }

    pub fn remain_ms(&mut self) -> i64 {
        let finish_timestamp = self.finish.timestamp_millis();
        let now_timestamp = Utc::now().timestamp_millis();
        finish_timestamp - now_timestamp
    }

    fn passed_ms(&mut self) -> i64 {
        let full_percent = self.finish.timestamp_millis() - self.start.timestamp_millis();
        let remain = self.remain_ms();
        full_percent - remain
    }
}
