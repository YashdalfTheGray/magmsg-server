use std::fs::{File, OpenOptions};

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
}
