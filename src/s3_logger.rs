use std::fs::File;

use rusoto_s3::S3Client;

use crate::log_line::LogLine;

pub struct S3Logger {
    file: Option<File>,
    debug: bool,
    log_lines: Vec<LogLine>,
    s3_client: S3Client,
}
