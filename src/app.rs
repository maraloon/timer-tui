use std::time::{Duration, Instant};

pub struct App {
    pub from: Instant,
    pub to: Instant,
    // timer: i64,
    // remained: i64,
    // finish_time: date,
}

impl App {

    pub fn new() -> App {
        let test_time = 10; // 10 sec
        let from = Instant::now();
        App {
            from,
            to: from - Duration::from_secs(test_time),
        }
    }
}
