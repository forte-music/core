#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate redis;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_redis;

pub mod schema;
pub mod database;
pub mod actions;
pub mod server;
