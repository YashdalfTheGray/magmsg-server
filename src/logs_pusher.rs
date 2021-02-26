use std::fs;

use chrono::{DateTime, Utc};
use log::{info, warn};
use rusoto_credential::AutoRefreshingProvider;

use crate::sdk::{self, CustomStsProvider};

use crate::appenv::*;

pub struct S3LogsPusher {
    bucket_name: String,
    last_successful_write: DateTime<Utc>,
    creds_provider: AutoRefreshingProvider<CustomStsProvider>,
}

impl S3LogsPusher {
    pub fn new(bucket_name: String) -> S3LogsPusher {
        S3LogsPusher {
            bucket_name,
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

    pub fn publish_to_s3(&mut self, target_file: String) {
        let s3client = sdk::get_s3_client(self.creds_provider.clone(), region());
        let contents = match fs::read_to_string(target_file.clone()) {
            Ok(file_str) => {
                info!(
                    "Reading application log to upload to S3, length: {}",
                    file_str.len()
                );
                file_str
            }
            Err(e) => {
                warn!("Reading application log failed!");
                warn!("{}", e);
                String::from("")
            }
        };

        self.last_successful_write = Utc::now();
    }
}
