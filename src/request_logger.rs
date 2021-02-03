use std::fs::{File, OpenOptions};

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
