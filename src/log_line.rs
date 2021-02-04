use std::net::IpAddr;

use chrono::{offset::Utc, DateTime, Duration};
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
        }
    }
}
