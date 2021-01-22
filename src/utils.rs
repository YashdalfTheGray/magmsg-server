use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;

pub fn get_time_in_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

pub fn get_time_to_expire(expiration_str: String) -> i64 {
    let expiration_time = DateTime::parse_from_rfc3339(&expiration_str).unwrap();
    let right_now = Utc::now();

    expiration_time.timestamp() - right_now.timestamp()
}

pub fn parse_into_utc(expiration_str: String) -> DateTime<Utc> {
    DateTime::from_utc(
        DateTime::parse_from_rfc3339(&expiration_str)
            .unwrap()
            .naive_utc(),
        Utc,
    )
}
