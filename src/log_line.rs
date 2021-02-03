use chrono::{offset::Utc, DateTime, Duration};

#[derive(Debug)]
pub struct LogLine {
    request_id: String,
    path: String,
    method: String,
    client_addr: String,
    received_at: DateTime<Utc>,
    responded_at: DateTime<Utc>,
    duration: Duration,
}
