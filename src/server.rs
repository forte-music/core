extern crate dotenv;

use std::env;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use iron::{Chain, Iron};
use mount::Mount;
use logger::Logger;
use schema::model::{Mutation, Query};
use context;

pub fn start() {
    let mut mount = Mount::new();
    let mut chain = Chain::new(mount);

    // Register Routes
    let graphql_handler = GraphQLHandler::new(context::IronContext::from_request, Query, Mutation);
    mount.mount("/graphql", graphql_handler);

    let graphiql_handler = GraphiQLHandler::new("/graphql");
    mount.mount("/", graphiql_handler);

    // Setup Context Middleware
    chain.link(context::init_context_middleware().expect("failed to initialize context"));

    // Setup Logging Middleware
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    // Start Server
    let host = dotenv::var("DATABASE_URL").map(|s| { s.as_str() }).unwrap_or("0.0.0.0:8080");
    println!("Starting Server on {}", host);
    let iron = Iron::new(chain);
    iron.http(host).unwrap();
}
