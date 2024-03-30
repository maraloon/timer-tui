use chrono::{DateTime, Duration, Local};

pub struct Timer {
    pub passed_ms: i64,
    pub remain_ms: i64,
    pub full_ms: i64,
    pub finish_at: DateTime<Local>,
    pub paused: bool,
    paused_at: DateTime<Local>, // TODO: Option
}

impl Timer {
    pub fn new(timer_duration: i64) -> Timer {
        let now = Local::now();
        let finish_at = now + Duration::milliseconds(timer_duration);
        let full_ms = finish_at.timestamp_millis() - now.timestamp_millis();
        let passed_ms = 0;
        let remain_ms = full_ms - passed_ms;
        Timer {
            passed_ms,
            remain_ms,
            full_ms,
            finish_at,
            paused: false,
            paused_at: now,
        }
    }

    pub fn tick(&mut self) {
        if self.paused {
            self.finish_at = Local::now() + Duration::milliseconds(self.remain_ms);
        } else {
            let now = Local::now();
            self.remain_ms = self.finish_at.timestamp_millis() - now.timestamp_millis();
            self.passed_ms = self.full_ms - self.remain_ms;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        if self.paused {
            self.paused_at = Local::now()
        }
    }
}
