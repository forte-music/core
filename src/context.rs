use diesel::sqlite::SqliteConnection;
use std::sync::{Mutex, MutexGuard};

pub type ConnectionManager = r2d2_diesel::ConnectionManager<SqliteConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub fn init_pool(database_url: &str) -> Result<Pool, r2d2::Error> {
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::new(manager)
}

pub struct GraphQLContext {
    connection: Mutex<PooledConnection>,
}

impl GraphQLContext {
    pub fn new(connection: PooledConnection) -> GraphQLContext {
        GraphQLContext {
            connection: Mutex::new(connection),
        }
    }

    pub fn connection(&self) -> MutexGuard<PooledConnection> {
        self.connection.lock().unwrap()
    }
}

impl juniper::Context for GraphQLContext {}
