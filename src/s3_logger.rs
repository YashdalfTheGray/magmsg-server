use std::{fs::OpenOptions, io::Write};

use chrono::{DateTime, Utc};
use rusoto_credential::AutoRefreshingProvider;

use crate::{
    log_line::LogLine,
    sdk::{self, CustomStsProvider},
};

use crate::appenv::*;

pub struct S3Logger {
    log_lines_cache: Vec<LogLine>,
    last_successful_write: DateTime<Utc>,
    creds_provider: AutoRefreshingProvider<CustomStsProvider>,
}

impl S3Logger {
    pub fn new() -> S3Logger {
        S3Logger {
            log_lines_cache: vec![],
            last_successful_write: Utc::now(),
            creds_provider: AutoRefreshingProvider::new(sdk::CustomStsProvider::new(
                assume_role_user_creds(),
                logging_assume_role_arn(),
                Some(external_id()),
                region(),
            ))
            .expect("Something went wrong while creating the logging creds provider"),
        }
    }

    pub fn log_request(&mut self, line: LogLine) {
        self.log_lines_cache.push(line.clone());
    }

    pub fn publish_to_s3(&mut self) {
        let s3client = sdk::get_s3_client(self.creds_provider.clone(), region());
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("logs/requests.log")
            .unwrap();

        let chunk_of_logs = self
            .log_lines_cache
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        file.write((chunk_of_logs + "\n").as_bytes()).unwrap();
    }
}
