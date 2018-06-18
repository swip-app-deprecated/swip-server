use actix::prelude::*;
use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use r2d2;
use std;
use dotenv;

// Pool database connection
pub fn get_db_connection_pool() -> DBPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Did not find DATABASE_URL in config");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Pool connection error")
}

// Get database adress
pub fn get_db_address(capacity: usize) -> actix::Addr<Syn, DbExecutor> {
    SyncArbiter::start(capacity, move || DbExecutor(get_db_connection_pool()))
}

// Create database pool type
pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;

// Create database executor from database pool
pub struct DbExecutor(DBPool);

// Add context to db executor and sync
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
