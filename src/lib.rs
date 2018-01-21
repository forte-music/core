#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate juniper;
#[macro_use] extern crate serde_derive;
extern crate juniper_rocket;
extern crate rocket;
extern crate redis;
extern crate serde;
extern crate serde_redis;
extern crate chrono;

pub mod schema;
pub mod database;
pub mod actions;
pub mod server;
