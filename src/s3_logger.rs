use std::fs::File;

use rusoto_s3::S3Client;

pub struct S3Logger {
    file: Option<File>,
    debug: bool,
    s3_client: S3Client,
}
