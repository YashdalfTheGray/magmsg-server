use std::env;

use rusoto_core::Region;

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
