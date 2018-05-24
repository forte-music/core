extern crate dotenv;
extern crate forte_core;
extern crate iron;
extern crate juniper_iron;
extern crate logger;
extern crate mount;

use forte_core::context::{GraphQLContext, IronContext};
use forte_core::models::{Mutation, Query};

use dotenv::dotenv;
use iron::{Chain, Iron};
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;
use std::env;
use std::error::Error;

fn main() {
    start().unwrap();
}

fn start() -> Result<(), Box<Error>> {
    dotenv().ok();

    let mut mount = Mount::new();

    // Register Routes
    let graphql_handler = GraphQLHandler::new(GraphQLContext::from_request, Query, Mutation);
    mount.mount("/graphql", graphql_handler);

    let graphiql_handler = GraphiQLHandler::new("/graphql");
    mount.mount("/", graphiql_handler);

    // Setup Context Middleware
    let mut chain = Chain::new(mount);
    let database_url = env::var("DATABASE_URL")?;
    chain.link(IronContext::init_middleware(&database_url)?);

    // Setup Logging Middleware
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    // Start Server
    // let host: &str =

    let host: String = match env::var("HOST") {
        Err(env::VarError::NotPresent) => None,
        Err(o) => return Err(Box::new(o)),
        Ok(s) => Some(s),
    }.unwrap_or("0.0.0.0:8080".to_owned());

    println!("Starting Server on {}", host);
    let iron = Iron::new(chain);
    iron.http(host)?;

    Ok(())
}
