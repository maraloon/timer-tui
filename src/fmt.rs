use chrono::{Duration, Timelike};

use crate::timer::Timer;

pub fn remain_time_string(timer: &mut Timer) -> String {
    let diff = Duration::milliseconds(timer.remain_ms() + 1000);
    format_duration(diff)
}

pub fn finish_time_string(timer: &mut Timer) -> String {
    let hour = timer.finish.hour();
    let minute = timer.finish.minute();
    format!("{:02}:{:02}", hour, minute)
}

fn format_duration(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    if hours == 0 {
        return format!("{:02}:{:02}", minutes, seconds);
    }
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
