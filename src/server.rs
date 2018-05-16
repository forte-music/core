extern crate dotenv;

use context;
use context::GraphQLContext;
use iron::{Chain, Iron};
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use models::{Mutation, Query};
use mount::Mount;

pub fn start() {
    let mut mount = Mount::new();

    // Register Routes
    let graphql_handler = GraphQLHandler::new(GraphQLContext::from_request, Query, Mutation);
    mount.mount("/graphql", graphql_handler);

    let graphiql_handler = GraphiQLHandler::new("/graphql");
    mount.mount("/", graphiql_handler);

    // Setup Context Middleware
    let mut chain = Chain::new(mount);
    chain.link(context::init_middleware().expect("failed to initialize context"));

    // Setup Logging Middleware
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    // Start Server
    let host = dotenv::var("HOST").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    println!("Starting Server on {}", host);
    let iron = Iron::new(chain);
    iron.http(host.as_str()).unwrap();
}
