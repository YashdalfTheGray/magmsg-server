use std::net::IpAddr;

use chrono::{offset::Utc, DateTime, Duration, NaiveDateTime};
use rocket::{http::Method, Request};

#[derive(Debug)]
pub struct LogLine {
    request_id: String,
    path: String,
    method: Method,
    client_addr: Option<IpAddr>,
    received_at: DateTime<Utc>,
    responded_at: DateTime<Utc>,
    duration: Duration,
    data_length: usize,
}

impl LogLine {
    pub fn empty() -> Self {
        LogLine {
            request_id: String::from(""),
            path: String::from(""),
            method: Method::Options,
            client_addr: None,
            received_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            responded_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            duration: Duration::seconds(0),
            data_length: 0,
        }
    }

    pub fn set_data_size(&mut self, len: usize) {
        self.data_length = len;
    }
}

impl From<Request<'_>> for LogLine {
    fn from(req: Request) -> Self {
        LogLine {
            request_id: req.local_cache(|| uuid::Uuid::nil().to_string()).clone(),
            path: req.uri().to_string(),
            method: req.method(),
            client_addr: req.client_ip(),
            received_at: Utc::now(),
            responded_at: Utc::now(),
            duration: Duration::seconds(0),
            data_length: 0,
        }
    }
}
