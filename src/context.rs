extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

use self::diesel::pg::PgConnection;

use std::ops::Deref;
use std::error::Error;
use iron::prelude::*;
use iron::typemap::Key;
use juniper;
use persistent::Read;
use iron::BeforeMiddleware;
use iron::AfterMiddleware;

pub type ConnectionManager = r2d2_diesel::ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<ConnectionManger>;

pub struct IronContext {
    pub connection_manager: r2d2::PolledConnectionManager<PgConnection>,
}

#[derive(Copy, Clone)]
pub struct ContextKey;
impl Key for ContextKey {
    type Value = IronContext;
}

impl IronContext {
    pub fn init_middleware() -> Result<(BeforeMiddleware, AfterMiddleware), Box<Error>> {
        let database_url = dotenv::var("DATABASE_URL")?;
        let manager = ConnectionManager::new(database_url)?;
        let pool: Pool = r2d2::Pool(manager)?;

        Read::<database::ConnectionKey>::both(pool)
    }
}

pub struct GraphQLContext {
    pub connection: r2d2::PooledConnection<r2d2_diesel::ConnectionManager<PgConnection>>,

}

impl GraphQLContext {
    pub fn from_request(request: &mut Request) -> Self {
        let iron_context = request.get::<Read<ContextKey>>().unwrap();
        let connection = iron_context.connection_manager.get();

        GraphQLContext {
            connection,
        }
    }
}

impl juniper::Context for GraphQLContext {}
