use crate::db::postgres_db::Connection;
use crate::schema::clients_schema::clients;
use diesel;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub profession: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Insertable)]
#[diesel(table_name = clients)]
#[diesel(check_for_backend(Pg))]
pub struct NewClient {
    pub name: String,
    pub password: String,
    pub profession: Option<String>,
}

impl Client {
    pub fn read(mut connection: Connection) -> Vec<Client> {
        // "&mut *connection" can be written as "connection.get_mut_connection()"
        let con = &mut *connection;
        clients::table
            .order(clients::id)
            .load::<Client>(con)
            .unwrap()
    }

    pub fn find_by_id(id: i32, mut connection: Connection) -> Client {
        let con = &mut *connection;
        clients::table.find(id).first(con).unwrap()
    }

    pub fn find_by_name(name: String, mut connection: Connection) -> Client {
        let con = &mut *connection;
        clients::table
            .filter(clients::name.eq(name))
            .first(con)
            .unwrap()
    }

    pub fn find_like_name(name: String, mut connection: Connection) -> Vec<Client> {
        let con = &mut *connection;
        clients::table
            .order(clients::id)
            .filter(clients::name.like(format!("%{}%", name)))
            .load::<Client>(con)
            .unwrap()
    }

    pub fn update(id: i32, c: Client, mut connection: Connection) -> bool {
        let con = &mut *connection;
        diesel::update(clients::table.find(id))
            .set(&c)
            .execute(con)
            .is_ok()
    }

    pub fn delete(id: i32, mut connection: Connection) -> bool {
        let con = &mut *connection;
        diesel::delete(clients::table.find(id)).execute(con).is_ok()
    }
}

impl NewClient {
    pub fn create(c: NewClient, mut connection: Connection) -> Client {
        let con = &mut *connection;
        diesel::insert_into(clients::table)
            .values(&c)
            .execute(con)
            .expect("Error creating new client");

        clients::table.order(clients::id.desc()).first(con).unwrap()
    }
}
