#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;

use std::sync::{mpsc, Mutex};
use std::thread;

use appenv::*;
use dotenv::dotenv;
use log::debug;
use log_line::LogLine;
use rocket::config::{Config, Environment};
use rocket_contrib::{helmet::SpaceHelmet, templates::Template};
use rusoto_credential::AutoRefreshingProvider;

mod appenv;
mod authenticator;
mod catchers;
mod constants;
mod dal;
mod log_line;
mod logs_pusher;
mod logs_writer;
mod message;
mod message_request;
mod request_id;
mod request_logger;
mod routes;
mod sdk;
mod utils;

fn main() {
    dotenv().ok();

    utils::configure_application_logging(appenv::application_log_path()).unwrap();

    let config = Config::build(Environment::Development)
        .port(port())
        .finalize()
        .unwrap();

    let app_creds_provider = sdk::CustomStsProvider::new(
        assume_role_user_creds(),
        assume_role_arn(),
        Some(external_id()),
        region(),
    );

    let auto_app_creds_provider = AutoRefreshingProvider::new(app_creds_provider)
        .expect("Something went wrong while creating the app creds provider");

    let (tx, rx) = mpsc::channel::<LogLine>();

    let request_logger_thread = thread::spawn(move || {
        debug!("Started file logger thread, waiting for request log statements.");
        let mut logger = logs_writer::LogsWriter::new(request_log_path());

        for line in rx {
            logger.log_request(line);
        }
    });

    let mutex_tx = Mutex::new(tx);

    let rocket_thread_handle = thread::spawn(move || {
        debug!("Launching rocket instance.");
        rocket::custom(config)
            .attach(SpaceHelmet::default())
            .attach(request_id::RequestId::default())
            .attach(request_logger::RequestLogger::new(mutex_tx, None))
            .attach(Template::fairing())
            .manage(auto_app_creds_provider)
            .mount(
                "/",
                routes![
                    routes::index,
                    routes::api_index,
                    routes::get_all_messages,
                    routes::get_all_messages_no_auth,
                    routes::add_new_message,
                    routes::add_new_message_no_auth,
                    routes::get_one_message,
                    routes::get_one_message_no_auth,
                ],
            )
            .register(catchers![
                catchers::bad_request,
                catchers::unauthorized,
                catchers::not_found,
                catchers::internal_error,
            ])
            .launch();
    });

    if appenv::logging_assume_role_arn().is_some() {
        let s3_logs_pusher_thread = thread::spawn(move || {
            debug!(
                "Started S3 logs pusher thread, sleeping for {} seconds before writing.",
                log_write_interval().num_seconds()
            );
            thread::sleep(log_write_interval().to_std().unwrap());
            let mut logs_pusher = logs_pusher::S3LogsPusher::new(logging_bucket_name());
            logs_pusher
                .publish_to_s3(&(request_log_path().clone()))
                .unwrap();
        });

        s3_logs_pusher_thread.join().unwrap();
    }

    rocket_thread_handle.join().unwrap();
    request_logger_thread.join().unwrap();
}
