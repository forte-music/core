extern crate r2d2;
extern crate r2d2_diesel;

use iron::prelude::*;
use iron::typemap::Key;
use persistent::Read;

use diesel::sqlite::SqliteConnection;
use juniper;

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

pub fn init_pool(database_url: &str) -> Result<Pool, r2d2::Error> {
    let manager = ConnectionManager::new(database_url);
    Ok(r2d2::Pool::new(manager)?)
}

impl IronContext {
    pub fn init_middleware(
        database_url: &str,
    ) -> Result<(Read<ContextKey>, Read<ContextKey>), r2d2::Error> {
        let pool = init_pool(database_url)?;
        let context = IronContext { pool };

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
