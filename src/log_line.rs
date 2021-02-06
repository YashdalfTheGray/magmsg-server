use std::fmt::{self, Formatter};
use std::net::IpAddr;

use chrono::{offset::Utc, DateTime, Duration, NaiveDateTime};
use rocket::{http::Method, Request};

#[derive(Debug, Clone)]
pub enum LogFormat {
    ApacheStandard,
    ApacheCommon,
    Dev,
    Short,
    Tiny,
    Default,
}

#[derive(Debug, Clone)]
pub struct LogLine {
    request_id: String,
    path: String,
    method: Method,
    client_addr: Option<IpAddr>,
    received_at: DateTime<Utc>,
    responded_at: DateTime<Utc>,
    duration: Duration,
    request_data_length: usize,
    response_data_length: usize,
    format: LogFormat,
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
            response_data_length: 0,
            format: LogFormat::Default,
        }
    }

    pub fn set_request_data_size(&mut self, len: usize) {
        self.request_data_length = len;
    }

    pub fn set_response_data_size(&mut self, len: usize) {
        self.response_data_length = len;
    }

    pub fn set_responded_at_time(&mut self, responded_at_time: DateTime<Utc>) {
        self.responded_at = responded_at_time;
        self.duration = self.responded_at.signed_duration_since(self.received_at);
    }

    pub fn set_responded_at_to_now(&mut self) {
        self.responded_at = Utc::now();
        self.duration = self.responded_at.signed_duration_since(self.received_at);
    }

    pub fn set_logging_format(&mut self, format: LogFormat) {
        self.format = format;
    }
}

impl fmt::Display for LogLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.format {
            LogFormat::ApacheCommon => write!(f, "ApacheCommon log line"),
            LogFormat::ApacheStandard => write!(f, "ApacheStandard log line"),
            LogFormat::Default => write!(f, "Default log line"),
            LogFormat::Dev => write!(f, "Dev log line"),
            LogFormat::Short => write!(f, "Short log line"),
            LogFormat::Tiny => write!(f, "Tiny log line"),
        }
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
            response_data_length: 0,
            format: LogFormat::Default,
        }
    }
}
