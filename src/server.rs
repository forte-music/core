use std::env;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use iron::{Chain, Iron};
use mount::Mount;
use logger::Logger;
use persistent::Read;
use schema::model::{Mutation, Query};
use database;

pub fn start() {
    let mut mount = Mount::new();
    let (logger_before, logger_after) = Logger::new(None);
    let db_pool = database::init_pool().expect("Could not connect to the database");

    let graphql_handler = GraphQLHandler::new(database::from_request, Query, Mutation);

    let graphiql_handler = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_handler);
    mount.mount("/graphql", graphql_handler);

    let mut chain = Chain::new(mount);
    chain.link(Read::<database::ConnectionKey>::both(db_pool));
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or("0.0.0.0:8000".to_owned());
    println!("Forte started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
