#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;

use dotenv::dotenv;
use rocket::config::{Config, Environment};
use std::env;

mod catchers;
mod routes;

fn main() {
    dotenv().ok();

    let config = Config::build(Environment::Development)
        .port(match env::var("PORT") {
            Ok(p) => p.parse::<u16>().unwrap_or(8080),
            Err(_e) => 8080,
        })
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount(
            "/",
            routes![
                routes::index,
                routes::api_index,
                routes::get_all_messages,
                routes::add_new_message,
                routes::get_one_message
            ],
        )
        .register(catchers![
            catchers::not_found,
            catchers::internal_error,
            catchers::bad_request
        ])
        .launch();
}
