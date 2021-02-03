use std::fs::{File, OpenOptions};

pub struct RequestLog {
    file: File,
}

impl RequestLog {
    pub fn new(filename: String) -> RequestLog {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .unwrap();

        RequestLog { file }
    }
}
