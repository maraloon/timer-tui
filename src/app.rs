use chrono::{prelude::*, DateTime, Duration, Local, Utc};

pub struct App {
    pub from: DateTime<Local>,
    pub to: DateTime<Local>,
}

impl App {
    pub fn new(first_arg: String) -> App {
        let mut app = App {
            from: Local::now(),
            to: Local::now()
        };

        let time = app.parse_time_argument(first_arg);
        app.from = Local::now();
        app.to = app.from + Duration::milliseconds(time);

        app
    }

    fn parse_time_argument(&mut self, time_string: String) -> i64 {
        let result: Result<i64, _> = time_string.parse();
        match result {
            Ok(_parsed_number) => {}
            Err(_) => {
                panic!("Failed to parse the string as an i64");
            }
        }
        result.unwrap() * 1000

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
        let hour = self.to.hour();
        let minute = self.to.minute();
        format!("{:02}:{:02}", hour, minute)
    }

    pub fn passed(&mut self) -> i64 {
        let full_percent = self.to.timestamp_millis() - self.from.timestamp_millis();
        let remain = self.remain();
        full_percent - remain
    }

    fn format_duration(&mut self, duration: Duration) -> String {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        if hours == 0 {
            return format!("{:02}:{:02}", minutes, seconds);
        }
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
