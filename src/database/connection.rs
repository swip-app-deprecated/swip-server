use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv;
use std;

// Pool database connection
pub fn get_db_connection_pool() -> DBPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Did not find DATABASE_URL in config");
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Pool connection error")
}

// Get database adress
pub fn get_db_address(capacity: usize) -> actix::Addr<Syn, DbExecutor> {
    SyncArbiter::start(capacity, move || DbExecutor(get_db_connection_pool()))
}

// Create database pool type
pub type DBPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

// Create database executor from database pool
pub struct DbExecutor(DBPool);

// Add context to db executor and sync
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
