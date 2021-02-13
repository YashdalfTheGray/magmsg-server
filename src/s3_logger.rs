use crate::log_line::LogLine;

pub struct S3Logger {
    log_lines_cache: Vec<LogLine>,
}

impl S3Logger {
    pub fn new() -> S3Logger {
        S3Logger {
            log_lines_cache: vec![],
        }
    }

    pub fn log_request(&mut self, line: LogLine) {
        self.log_lines_cache.push(line);
    }
}
