use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Logger;

impl Logger {
    pub fn log(msg: impl Display) {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                eprintln!("Failed to get system time when creating a log");
                return;
            }
        };
        println!("{}: {}", timestamp, msg);
    }
}
