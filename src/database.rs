extern crate r2d2;
extern crate r2d2_redis;

use std::ops::Deref;
use self::r2d2_redis::RedisConnectionManager;
use redis;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use juniper;

pub type Pool = r2d2::Pool<RedisConnectionManager>;

pub fn init_pool() -> Pool {
    let manager = RedisConnectionManager::new("redis://127.0.0.1/").unwrap();
    r2d2::Pool::new(manager).unwrap()
}

pub struct Connection(r2d2::PooledConnection<RedisConnectionManager>);

impl juniper::Context for Connection {}

impl Deref for Connection {
    type Target = redis::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
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