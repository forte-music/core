extern crate r2d2;
extern crate r2d2_diesel;

use diesel::sqlite::SqliteConnection;
use dotenv;
use iron::prelude::*;
use iron::typemap::Key;
use juniper;
use persistent::Read;
use std::error::Error;
use std::ops::Deref;

pub type ConnectionManager = r2d2_diesel::ConnectionManager<SqliteConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub struct IronContext {
    pub pool: Pool,
}

#[derive(Copy, Clone)]
pub struct ContextKey;
impl Key for ContextKey {
    type Value = IronContext;
}

impl IronContext {
    pub fn init_middleware() -> Result<(Read<ContextKey>, Read<ContextKey>), Box<Error>> {
        let database_url = dotenv::var("DATABASE_URL")?;
        let manager = ConnectionManager::new(database_url);
        let pool: Pool = r2d2::Pool::new(manager)?;
        let context = IronContext { pool: pool };

        Ok(Read::<ContextKey>::both(context))
    }
}

pub struct GraphQLContext {
    connection: PooledConnection,
}

impl GraphQLContext {
    pub fn from_request(request: &mut Request) -> Self {
        let iron_context = request.get::<Read<ContextKey>>().unwrap();
        let connection: PooledConnection = iron_context.pool.get().unwrap();

        GraphQLContext { connection }
    }

    pub fn connection(&self) -> &SqliteConnection {
        self.connection.deref()
    }
}

impl juniper::Context for GraphQLContext {}
