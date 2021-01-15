use std::env;

pub fn port() -> u16 {
    match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap_or(8080),
        Err(_e) => 8080,
    }
}
