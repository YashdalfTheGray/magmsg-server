use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::log_line::LogLine;

pub struct LogsWriter {
    file: File,
}

impl LogsWriter {
    pub fn new(file_path: String) -> LogsWriter {
        LogsWriter {
            file: OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path)
                .unwrap(),
        }
    }

    pub fn log_request(&mut self, line: LogLine) {
        self.file.write(line.to_string().as_bytes()).unwrap();
    }
}
