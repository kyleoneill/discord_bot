use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_unix_epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock rolled backwards")
        .as_secs() as i64
}

pub fn seconds_to_human_readable(seconds: i64) -> String {
    // This is pretty ugly, is there a way to write this nicer?
    let hours = if seconds >= 3600 {
        (seconds as f64 / 3600f64).floor() as i64
    } else {
        0i64
    };
    let remainder_minutes_as_seconds = seconds % 3600;
    let minutes = if remainder_minutes_as_seconds >= 60 {
        (remainder_minutes_as_seconds as f64 / 60f64).floor() as i64
    } else {
        0i64
    };
    let seconds = remainder_minutes_as_seconds % 60;
    if hours > 0 {
        return format!(
            "{} hours, {} minutes, and {} seconds",
            hours, minutes, seconds
        );
    }
    if minutes > 0 {
        return format!("{} minutes and {} seconds", minutes, seconds);
    }
    format!("{} seconds", seconds)
}
