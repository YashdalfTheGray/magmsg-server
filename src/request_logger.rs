use std::fs::{File, OpenOptions};

use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

use crate::log_line::LogLine;

#[derive(Debug)]
pub struct RequestLogger {
    file: File,
}

impl RequestLogger {
    pub fn new(filename: String) -> RequestLogger {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .unwrap();

        RequestLogger { file }
    }
}

impl Default for RequestLogger {
    fn default() -> Self {
        RequestLogger::new(String::from("logs/requests.log"))
    }
}

impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, data: &Data) {
        request.local_cache(|| LogLine::from(request.clone()));
    }

    fn on_response(&self, request: &Request, response: &mut Response) {}
}
