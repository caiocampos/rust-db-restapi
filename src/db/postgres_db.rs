use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::http::Status;
use rocket::outcome::{try_outcome, Outcome};
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use std::ops::{Deref, DerefMut};

// An alias to the type for a pool of Diesel Postgres Connection
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initialize the database pool.
pub fn connect(database: String, pool_size: u32) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database);
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool.
/// If no pool is currently managed, fails with an `InternalServerError` status.
/// If no connections are available, fails with a `ServiceUnavailable` status.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = try_outcome!(request.guard::<&State<PgPool>>().await);
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Error((Status::ServiceUnavailable, ())),
        }
    }
}

impl Connection {
    pub fn get_connection(&self) -> &PgConnection {
        &self.0
    }

    pub fn get_mut_connection(&mut self) -> &mut PgConnection {
        &mut self.0
    }
}

// For the convenience of using an &Connection as an &PgConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// For the convenience of using an &mut Connection as an &mut PgConnection.
impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
