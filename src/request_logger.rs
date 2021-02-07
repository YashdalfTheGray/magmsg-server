use std::fs::{File, OpenOptions};

use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

use crate::{
    appenv,
    log_line::{LogFormat, LogLine},
};

#[derive(Debug)]
pub struct RequestLogger {
    file: File,
    format: LogFormat,
}

impl RequestLogger {
    pub fn new(filename: String, format: LogFormat) -> RequestLogger {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .unwrap();

        RequestLogger { file, format }
    }
}

impl Default for RequestLogger {
    fn default() -> Self {
        RequestLogger::new(String::from("logs/requests.log"), appenv::log_format())
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
        let mut log_line = LogLine::from(request.clone());
        log_line.set_logging_format(self.format.clone());
        log_line.set_request_data_size(data.peek().len());
        request.local_cache(|| log_line);
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let mut log_line = request.local_cache(|| LogLine::empty()).clone();
        log_line.set_responded_at_to_now();
        log_line.set_response_data_size(match response.body_string() {
            Some(str) => str.len(),
            None => 0,
        });
        log_line.set_status(response.status());
        println!("{:#?}", log_line);
    }
}
