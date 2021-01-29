#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;

use dotenv::dotenv;
use rocket::config::{Config, Environment};
use rusoto_credential::AutoRefreshingProvider;

mod appenv;
mod authenticator;
mod catchers;
mod constants;
mod dal;
mod message;
mod message_request;
mod routes;
mod sdk;
mod utils;

fn main() {
    dotenv().ok();

    let config = Config::build(Environment::Development)
        .port(appenv::port())
        .finalize()
        .unwrap();

    let creds_provider = sdk::CustomStsProvider::new(
        appenv::assume_role_user_creds(),
        appenv::assume_role_arn(),
        Some(appenv::external_id()),
        appenv::region(),
    );

    let auto_creds_provider = AutoRefreshingProvider::new(creds_provider)
        .expect("Something went wrong while crating a creds provider");

    rocket::custom(config)
        .manage(auto_creds_provider)
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
