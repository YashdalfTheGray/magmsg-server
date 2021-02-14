use std::{
    io::Cursor,
    sync::{mpsc::Sender, Mutex},
};

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
    format: LogFormat,
    mutex_tx: Mutex<Sender<LogLine>>,
}

impl RequestLogger {
    pub fn new(
        mutex_tx: Mutex<Sender<LogLine>>,
        optional_format: Option<LogFormat>,
    ) -> RequestLogger {
        RequestLogger {
            mutex_tx,
            format: match optional_format {
                Some(format) => format,
                None => appenv::log_format(),
            },
        }
    }
}

impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _data: &Data) {
        let mut log_line = LogLine::from(request.clone());

        log_line.set_logging_format(self.format.clone());

        request.local_cache(|| log_line);
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let mut log_line = request.local_cache(|| LogLine::empty()).clone();
        let body_str = match response.body_string() {
            Some(body) => body,
            None => String::from(""),
        };

        log_line.set_response_data_size(body_str.clone().len());
        log_line.set_responded_at_to_now();
        log_line.set_status(response.status());

        response.set_sized_body(Cursor::new(body_str));

        match self.mutex_tx.lock() {
            Ok(tx) => {
                tx.send(log_line).unwrap();
            }
            Err(_) => (),
        }
    }
}
