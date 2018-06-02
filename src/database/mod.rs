use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::ops::Deref;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().expect("No .env file found");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set, please update the .env file accordingly");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Could not create Mysql database connection to: {}", database_url))
}

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Pool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let config = r2d2::Config::default();
    let manager = ConnectionManager::<MysqlConnection>::new(&database_url[..]);
    r2d2::Pool::new(config, manager)
        .expect(&format!("Error connecting to {}", &database_url[..]))
}

pub struct Connection(r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>)
        -> request::Outcome<Connection, ()> {

        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
