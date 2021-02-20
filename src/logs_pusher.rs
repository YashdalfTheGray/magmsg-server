use chrono::{DateTime, Utc};
use rusoto_credential::AutoRefreshingProvider;

use crate::sdk::{self, CustomStsProvider};

use crate::appenv::*;

pub struct S3LogsPusher {
    target_file: String,
    bucket_name: String,
    last_successful_write: DateTime<Utc>,
    creds_provider: AutoRefreshingProvider<CustomStsProvider>,
}

impl S3LogsPusher {
    pub fn new(target_file: String, bucket_name: String) -> S3LogsPusher {
        S3LogsPusher {
            target_file,
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

    pub fn publish_to_s3(&mut self) {
        let s3client = sdk::get_s3_client(self.creds_provider.clone(), region());
    }
}