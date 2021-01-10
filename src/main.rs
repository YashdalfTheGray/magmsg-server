#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod catchers;
mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::index])
        .register(catchers![catchers::not_found])
        .launch();
}
