use crate::db::postgres_db::Connection;
use crate::repository::client_repository::{Client, NewClient};
use rocket::serde::json::{json, Json, Value};
use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![
        read,
        find_by_id,
        find_by_name,
        find_like_name,
        create,
        update,
        delete
    ]
}

#[post("/", data = "<client>")]
fn create(client: Json<NewClient>, connection: Connection) -> Json<Client> {
    let insert = NewClient {
        ..client.into_inner()
    };
    Json(NewClient::create(insert, connection))
}

#[get("/")]
fn read(connection: Connection) -> Json<Vec<Client>> {
    Json(Client::read(connection))
}

#[get("/id/<id>")]
fn find_by_id(id: i32, connection: Connection) -> Json<Client> {
    Json(Client::find_by_id(id, connection))
}

#[get("/name/<name>")]
fn find_by_name(name: String, connection: Connection) -> Json<Client> {
    Json(Client::find_by_name(name, connection))
}

#[get("/s/name/<name>")]
fn find_like_name(name: String, connection: Connection) -> Json<Vec<Client>> {
    Json(Client::find_like_name(name, connection))
}

#[put("/<id>", data = "<client>")]
fn update(id: i32, client: Json<Client>, connection: Connection) -> Json<Value> {
    let update = Client {
        ..client.into_inner()
    };
    Json(json!({
        "success": Client::update(id, update, connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: Connection) -> Json<Value> {
    Json(json!({ "success": Client::delete(id, connection) }))
}
