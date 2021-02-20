use std::{env, str::FromStr};

use chrono::Duration;
use rusoto_core::Region;
use rusoto_credential::StaticProvider;

use crate::log_line::LogFormat;

pub fn port() -> u16 {
    match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap_or(8080),
        Err(_e) => 8080,
    }
}

pub fn region() -> Region {
    match env::var("AWS_REGION") {
        Ok(r) => r.parse::<Region>().unwrap_or(Region::UsEast2),
        Err(_e) => Region::UsEast2,
    }
}

pub fn table_name() -> String {
    match env::var("AWS_DYNAMO_DB_TABLE_NAME") {
        Ok(t) => t,
        Err(_e) => "messages".to_string(),
    }
}

pub fn assume_role_user_creds() -> StaticProvider {
    let aws_access_key = env::var("AWS_ACCESS_KEY_ID")
        .expect("Environment variable AWS_ACCESS_KEY_ID is required to be defined.");
    let aws_secret_key = env::var("AWS_SECRET_ACCESS_KEY")
        .expect("Environment variable AWS_SECRET_ACCESS_KEY is required to be defined.");

    StaticProvider::new_minimal(aws_access_key, aws_secret_key)
}

pub fn external_id() -> String {
    env::var("EXTERNAL_ID").expect("Environment variable EXTERNAL_ID is required to be defined.")
}

pub fn assume_role_arn() -> String {
    env::var("AWS_ASSUME_ROLE_ARN")
        .expect("Environment variable AWS_ASSUME_ROLE_ARN is required to be defined.")
}

pub fn logging_assume_role_arn() -> String {
    env::var("LOGGING_ASSUME_ROLE_ARN").unwrap_or_default()
}

pub fn user_access_token() -> String {
    env::var("USER_ACCESS_TOKEN")
        .expect("Environment variable USER_ACCESS_TOKEN is required to be defined.")
}

pub fn auth_header_key() -> String {
    env::var("AUTH_HEADER_KEY")
        .expect("Environment variable AUTH_HEADER_KEY is required to be defined.")
}

pub fn logging_bucket_name() -> String {
    match env::var("LOGGING_BUCKET_NAME") {
        Ok(bucket) => bucket,
        Err(_) => String::from("request_logs"),
    }
}

pub fn log_format() -> LogFormat {
    match env::var("LOG_FORMAT") {
        Ok(format) => match &(format.to_lowercase())[..] {
            "standard" => LogFormat::Standard,
            "dev" => LogFormat::Dev,
            "short" => LogFormat::Short,
            "tiny" => LogFormat::Tiny,
            "" | &_ => LogFormat::Standard,
        },
        Err(_) => LogFormat::Standard,
    }
}

pub fn log_write_interval() -> Duration {
    match env::var("LOG_WRITE_INTERVAL") {
        Ok(interval_str) => {
            let mut interval = interval_str.parse::<i64>().unwrap_or(60 * 60);

            if interval < 60 {
                interval = 60;
            }

            Duration::seconds(interval)
        }
        Err(_) => Duration::seconds(60 * 60),
    }
}

pub fn application_log_path() -> String {
    match env::var("APPLICATION_LOG_PATH") {
        Ok(path) => path,
        Err(_) => String::from("logs/application.log"),
    }
}

pub fn request_log_path() -> String {
    match env::var("REQUEST_LOG_PATH") {
        Ok(path) => path,
        Err(_) => String::from("logs/request.log"),
    }
}

pub fn log_level() -> log::LevelFilter {
    match env::var("LOG_LEVEL") {
        Ok(level_str) => match log::LevelFilter::from_str(&level_str) {
            Ok(level) => level,
            Err(_) => log::LevelFilter::Info,
        },
        Err(_) => log::LevelFilter::Info,
    }
}
