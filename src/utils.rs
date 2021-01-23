use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use rusoto_dynamodb::AttributeValue;

pub fn get_time_in_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

pub fn get_time_to_expire(expiration_str: String) -> i64 {
    let expiration_time = DateTime::parse_from_rfc3339(&expiration_str).unwrap();
    let right_now = Utc::now();

    expiration_time.timestamp() - right_now.timestamp()
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
