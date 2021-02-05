use std::fmt::{self, Formatter};
use std::net::IpAddr;

use chrono::{offset::Utc, DateTime, Duration, NaiveDateTime};
use rocket::{http::Method, Request};

pub enum LogFormat {
    ApacheStandard,
    ApacheCommon,
    Dev,
    Short,
    Tiny,
}

#[derive(Debug)]
pub struct LogLine {
    request_id: String,
    path: String,
    method: Method,
    client_addr: Option<IpAddr>,
    received_at: DateTime<Utc>,
    responded_at: DateTime<Utc>,
    duration: Duration,
    request_data_length: usize,
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
            request_data_length: 0,
        }
    }

    pub fn set_request_data_size(&mut self, len: usize) {
        self.request_data_length = len;
    }
}

impl fmt::Display for LogLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "log line goes here")
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
            request_data_length: 0,
        }
    }
}
