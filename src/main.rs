extern crate toml;

#[macro_use]
extern crate rocket;

extern crate diesel;

mod config;
mod controller;
mod db;
mod repository;
mod schema;

use config::toml_config::read_toml;
use controller::client_controller::get_routes;
use db::postgres_db::connect;

#[launch]
fn rocket() -> _ {
    let config = read_toml();
    let db = config.db();

    rocket::build()
        .manage(connect(db.url(), db.pool_size()))
        .mount("/user", get_routes())
}
