extern crate r2d2;
extern crate r2d2_redis;

use std::ops::Deref;
use std::error::Error;
use self::r2d2_redis::RedisConnectionManager;
use redis;
use iron::prelude::*;
use iron::typemap::Key;
use persistent::Read;
use juniper;

pub type Pool = r2d2::Pool<RedisConnectionManager>;

pub fn init_pool() -> Result<Pool, Box<Error>> {
    let manager = RedisConnectionManager::new("redis://127.0.0.1/")?;
    Ok(r2d2::Pool::new(manager)?)
}

pub fn from_request(request: &mut Request) -> Connection {
    let pool = request.get::<Read<ConnectionKey>>().unwrap();
    Connection(pool.get().unwrap())
}

#[derive(Copy, Clone)]
pub struct ConnectionKey;
impl Key for ConnectionKey {
    type Value = Pool;
}

pub struct Connection(r2d2::PooledConnection<RedisConnectionManager>);

impl juniper::Context for Connection {}

impl Deref for Connection {
    type Target = redis::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
