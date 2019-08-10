#![feature(proc_macro_hygiene, decl_macro)]

extern crate toml;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod config;
mod controller;
mod db;
mod repository;
mod schema;

use config::toml_config::read_toml;
use controller::client_controller::get_routes;
use db::postgres_db::connect;

fn main() {
    let config = read_toml();
    let db = config.db();

    rocket::ignite()
        .manage(connect(db.url(), db.pool_size()))
        .mount("/user", get_routes())
        .launch();
}
