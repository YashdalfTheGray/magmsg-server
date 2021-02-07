use std::fmt::{self, Formatter};
use std::net::IpAddr;

use chrono::{offset::Utc, DateTime, Duration, NaiveDateTime};
use rocket::{
    http::{Method, Status},
    Request,
};

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
    client_addr: Option<IpAddr>,
    duration: Duration,
    format: LogFormat,
    method: Method,
    path: String,
    received_at: DateTime<Utc>,
    request_data_length: usize,
    request_id: String,
    responded_at: DateTime<Utc>,
    response_data_length: usize,
    status: Status,
}

impl LogLine {
    pub fn empty() -> Self {
        LogLine {
            client_addr: None,
            duration: Duration::seconds(0),
            format: LogFormat::Default,
            method: Method::Options,
            path: String::from(""),
            received_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            request_data_length: 0,
            request_id: String::from(""),
            responded_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            response_data_length: 0,
            status: Status::Ok,
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
        self.set_responded_at_time(Utc::now());
    }

    pub fn set_logging_format(&mut self, format: LogFormat) {
        self.format = format;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

impl fmt::Display for LogLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.format {
            LogFormat::ApacheCommon => write!(f, ":remote-addr - :remote-user [:date[clf]] \":method :url\" :status :res[content-length]"),
            LogFormat::ApacheStandard => write!(f, ":remote-addr - :remote-user [:date[clf]] \":method :url\" :status :res[content-length] \":referrer\" \":user-agent\""),
            LogFormat::Default => write!(f, "Default log line"),
            LogFormat::Dev => write!(f, ":method :url :status :response-time ms - :res[content-length]"),
            LogFormat::Short => write!(f, ":remote-addr :remote-user :method :url :status :res[content-length] - :response-time ms"),
            LogFormat::Tiny => write!(f, ":method :url :status :res[content-length] - :response-time ms"),
        }
    }
}

impl From<Request<'_>> for LogLine {
    fn from(req: Request) -> Self {
        LogLine {
            client_addr: req.client_ip(),
            duration: Duration::seconds(0),
            format: LogFormat::Default,
            method: req.method(),
            path: req.uri().to_string(),
            received_at: Utc::now(),
            request_data_length: 0,
            request_id: req.local_cache(|| uuid::Uuid::nil().to_string()).clone(),
            responded_at: Utc::now(),
            response_data_length: 0,
            status: Status::Ok,
        }
    }
}
