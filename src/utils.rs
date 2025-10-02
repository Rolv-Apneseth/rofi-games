use std::time::{SystemTime, UNIX_EPOCH};

/// Current Unix timestamp in seconds - based on system time.
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("problem with system clock")
        .as_secs()
}
