use chrono::{offset::Utc, DateTime, Duration};
use rocket::Request;

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

impl From<Request<'_>> for LogLine {
    fn from(req: Request) -> Self {
        LogLine {
            request_id: String::from("some string"),
            path: String::from("some string"),
            method: String::from("some string"),
            client_addr: String::from("some string"),
            received_at: Utc::now(),
            responded_at: Utc::now(),
            duration: Duration::seconds(10),
        }
    }
}
