extern crate diesel;
extern crate dotenv;
extern crate forte_core;
extern crate iron;
extern crate juniper_iron;
extern crate logger;
extern crate persistent;
extern crate router;
extern crate r2d2;

#[macro_use]
extern crate error_chain;

use diesel::result;

use forte_core::context::{GraphQLContext, IronContext};
use forte_core::models::{Mutation, Query, Song, UUID};

use iron::IronError;
use iron::IronResult;
use iron::Request;
use iron::Response;
use iron::{Chain, Iron};

use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use router::Router;

use dotenv::dotenv;
use iron::status;
use std::env;
use std::ops::Deref;

error_chain! {
    foreign_links {
        R2d2(::r2d2::Error);
        VarError(::std::env::VarError);
    }
}

fn main() {
    start().unwrap();
}

fn raw_handler(req: &mut Request) -> IronResult<Response> {
    let ctx = GraphQLContext::from_request(req);
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();

    let uuid = UUID::parse_str(id)
        .map_err(|err| IronError::new(err, ("invalid uuid", status::BadRequest)))?;

    let song = Song::from_id(&ctx, &uuid).map_err(|err| match err {
        result::Error::NotFound => IronError::new(err, status::NotFound),
        _ => IronError::new(err, status::InternalServerError),
    })?;

    Ok(Response::with((status::Ok, song.path.deref().clone())))
}

fn start() -> Result<()> {
    dotenv().ok();

    let mut router = Router::new();

    // Register Routes
    let graphql_handler = GraphQLHandler::new(GraphQLContext::from_request, Query, Mutation);
    router.any("/graphql", graphql_handler, "graphql");

    let graphiql_handler = GraphiQLHandler::new("/graphql");
    router.any("/", graphiql_handler, "graphiql");

    router.get("/files/music/:id/raw", raw_handler, "raw");

    // Setup Context Middleware
    let mut chain = Chain::new(router);
    let database_url = env::var("DATABASE_URL")?;
    chain.link(IronContext::init_middleware(&database_url)?);

    // Setup Logging Middleware
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    // Start Server
    let host: String = match env::var("HOST") {
        Err(env::VarError::NotPresent) => None,
        Err(o) => return Err(o.into()),
        Ok(s) => Some(s),
    }.unwrap_or("0.0.0.0:8080".to_owned());

    println!("Starting Server on {}", host);
    let iron = Iron::new(chain);
    iron.http(host).unwrap();

    Ok(())
}
