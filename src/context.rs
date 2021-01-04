use diesel::sqlite::SqliteConnection;
use send_wrapper::SendWrapper;

pub type ConnectionManager = r2d2_diesel::ConnectionManager<SqliteConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub fn init_pool(database_url: &str) -> Result<Pool, r2d2::Error> {
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::new(manager)
}

pub struct GraphQLContext {
    // Wrap the connection in SendWrapper since actix ensures futures don't move
    // across threads. SendWrapper makes the context Send + Sync.
    connection: SendWrapper<PooledConnection>,
}

impl GraphQLContext {
    pub fn new(connection: PooledConnection) -> GraphQLContext {
        GraphQLContext {
            connection: SendWrapper::new(connection),
        }
    }

    pub fn connection(&self) -> &SqliteConnection {
        &self.connection
    }
}

impl juniper::Context for GraphQLContext {}
