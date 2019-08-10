use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::client_schema::client;

#[table_name = "client"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
    pub profession: Option<String>,
}

impl Client {
    pub fn create(c: Client, con: &PgConnection) -> Client {
        diesel::insert_into(client::table)
            .values(&c)
            .execute(con)
            .expect("Error creating new client");

        client::table.order(client::id.desc()).first(con).unwrap()
    }

    pub fn read(con: &PgConnection) -> Vec<Client> {
        client::table.order(client::id).load::<Client>(con).unwrap()
    }

    pub fn find_by_id(id: i32, con: &PgConnection) -> Client {
        client::table.find(id).first(con).unwrap()
    }

    pub fn find_by_name(name: String, con: &PgConnection) -> Client {
        client::table
            .filter(client::name.eq(name))
            .first(con)
            .unwrap()
    }

    pub fn find_like_name(name: String, con: &PgConnection) -> Vec<Client> {
        client::table
            .order(client::id)
            .filter(client::name.like(format!("%{}%", name)))
            .load::<Client>(con)
            .unwrap()
    }

    pub fn update(id: i32, c: Client, con: &PgConnection) -> bool {
        diesel::update(client::table.find(id))
            .set(&c)
            .execute(con)
            .is_ok()
    }

    pub fn delete(id: i32, con: &PgConnection) -> bool {
        diesel::delete(client::table.find(id)).execute(con).is_ok()
    }
}
