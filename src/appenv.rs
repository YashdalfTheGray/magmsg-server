use std::env;

use rusoto_core::Region;
use rusoto_credential::StaticProvider;

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
    let aws_access_key = env::var("AWS_ACCESS_KEY_ID").unwrap();
    let aws_secret_key = env::var("AWS_SECRET_ACCESS_KEY").unwrap();

    StaticProvider::new_minimal(aws_access_key, aws_secret_key)
}

pub fn external_id() -> String {
    env::var("EXTERNAL_ID").unwrap()
}

pub fn assume_role_arn() -> String {
    env::var("AWS_ASSUME_ROLE_ARN").unwrap()
}

pub fn user_access_token() -> String {
    env::var("USER_ACCESS_TOKEN").unwrap()
}
