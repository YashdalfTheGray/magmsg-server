use std::fs::read_to_string;

use chrono::{DateTime, Utc};
use log::{info, warn};
use rusoto_core::RusotoError;
use rusoto_credential::AutoRefreshingProvider;
use rusoto_s3::{PutObjectError, PutObjectOutput};

use crate::appenv::*;
use crate::sdk::{self, CustomStsProvider};
use crate::utils;

pub struct S3LogsPusher {
    bucket_name: String,
    last_successful_write: DateTime<Utc>,
    creds_provider: AutoRefreshingProvider<CustomStsProvider>,
}

impl S3LogsPusher {
    pub fn new(bucket_name: String) -> S3LogsPusher {
        // When this code gets executed, we have already checked that
        // logging_assume_role_arn() will return a Some() so it is safe
        // to unwrap it here!
        S3LogsPusher {
            bucket_name,
            last_successful_write: Utc::now(),
            creds_provider: AutoRefreshingProvider::new(CustomStsProvider::new(
                assume_role_user_creds(),
                logging_assume_role_arn().unwrap(),
                Some(external_id()),
                region(),
            ))
            .expect("Something went wrong while creating the logging creds provider"),
        }
    }

    pub fn publish_to_s3(
        &mut self,
        target_file: &String,
    ) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
        let s3client = sdk::get_s3_client(self.creds_provider.clone(), region());
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let contents = match read_to_string(target_file.clone()) {
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

        let put_object_future = crate::dal::put_object(
            s3client,
            self.bucket_name.clone(),
            utils::build_s3_object_key(target_file.clone()),
            contents,
        );
        let put_object = runtime.block_on(put_object_future);

        self.last_successful_write = Utc::now();

        put_object
    }
}
