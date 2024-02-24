use chrono::{prelude::*, DateTime, Utc, Duration};
// use std::time::Duration;

pub struct App {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl App {
    pub fn new() -> App {
        let test_time = 13000;
        let from = Utc::now();
        // let to = from + Duration::from_millis(test_time);
        let to = from + Duration::milliseconds(test_time);

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

    pub fn remain_seconds(&mut self) -> String {
        // todo + 1000 looks dirty
        let diff = Duration::milliseconds(self.remain() + 1000);
        self.format_duration(diff)
    }

    pub fn bell_time(&mut self) -> String {
        let when = Duration::milliseconds(self.to.timestamp_millis());
        self.format_duration(when)
    }

    pub fn passed(&mut self) -> i64 {
        let full_percent = self.to.timestamp_millis() - self.from.timestamp_millis();
        let remain = self.remain();
        full_percent - remain
    }

    fn format_duration(&mut self,duration: Duration) -> String {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        if hours == 0 {
            return format!("{:02}:{:02}", minutes, seconds);
        }
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

