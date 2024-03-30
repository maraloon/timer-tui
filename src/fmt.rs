use chrono::{Duration, Timelike};

use crate::timer::Timer;

pub fn remain_time_string(timer: &mut Timer) -> String {
    // when we start timer at 15s, achualy it's 14,99999 right after starting
    // and it's look like a bug, when we show 14, not 15
    // so i add 1 second
    // to fake reality and don't fear users with horrible truth
    let remain_ms_fixuped = timer.remain_ms + 1000;

    let remain = Duration::milliseconds(remain_ms_fixuped);

    let hours = remain.num_hours();
    let minutes = remain.num_minutes() % 60;
    let seconds = remain.num_seconds() % 60;

    if hours == 0 {
        return format!("{:02}:{:02}", minutes, seconds);
    }
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn finish_time_string(timer: &mut Timer) -> String {
    let hour = timer.finish_at.hour();
    let minute = timer.finish_at.minute();
    let second = timer.finish_at.second();
    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

pub fn ratio(timer: &mut Timer) -> f64 {
    timer.passed_ms as f64 / timer.full_ms as f64
}
