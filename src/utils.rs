use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use rocket_contrib::json::JsonValue;
use rusoto_dynamodb::AttributeValue;

use crate::appenv::*;

pub fn get_time_in_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

pub fn parse_into_utc(expiration_str: String) -> DateTime<Utc> {
    DateTime::from_utc(
        DateTime::parse_from_rfc3339(&expiration_str)
            .unwrap()
            .naive_utc(),
        Utc,
    )
}

pub fn get_string_from_attribute_value(attr: Option<&AttributeValue>) -> String {
    attr.map_or_else(
        || String::default(),
        |val| val.s.clone().unwrap_or_default(),
    )
}

pub fn get_number_from_attribute_value(attr: Option<&AttributeValue>) -> u128 {
    attr.map_or_else(
        || u128::default(),
        |val| {
            val.n
                .clone()
                .unwrap_or_default()
                .parse::<u128>()
                .unwrap_or_default()
        },
    )
}

pub fn wrap_string_in_attribute_value(str: String) -> AttributeValue {
    AttributeValue {
        s: Some(str),
        ..AttributeValue::default()
    }
}

pub fn wrap_number_in_attribute_value(num: u128) -> AttributeValue {
    AttributeValue {
        n: Some(num.to_string()),
        ..AttributeValue::default()
    }
}

pub fn determine_status() -> JsonValue {
    let creds = assume_role_user_creds();
    let user_found =
        !creds.get_aws_access_key_id().is_empty() && !creds.get_aws_secret_access_key().is_empty();
    let auth_found = !auth_header_key().is_empty() && !user_access_token().is_empty();
    let creds_found = user_found && !assume_role_arn().is_empty();
    let table_found = !table_name().is_empty() && !region().name().is_empty();

    json!({
        "status": if auth_found && creds_found && table_found {
            "okay"
        } else {
            "error"
        }
    })
}

pub fn configure_application_logging(path: String) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .level(log_level())
                .chain(fern::DateBased::new(path, ".%Y-%m-%d-%H")),
        )
        .chain(
            fern::Dispatch::new()
                .level(log_level())
                .chain(std::io::stdout()),
        )
        .apply()?;
    Ok(())
}
