extern crate iron;
extern crate juniper_iron;
extern crate logger;
extern crate persistent;
extern crate router;

use diesel::result;

use forte_core::context;
use forte_core::context::{GraphQLContext, IronContext};
use forte_core::models::{Mutation, Query, Song, UUID};

use self::iron::IronError;
use self::iron::IronResult;
use self::iron::Request;
use self::iron::Response;
use self::iron::status;
use self::iron::{Chain, Iron};

use self::juniper_iron::{GraphQLHandler, GraphiQLHandler};
use self::logger::Logger;
use self::router::Router;

use std::ops::Deref;

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

pub fn serve(pool: context::Pool, host: String) {
    let mut router = Router::new();

    // Register Routes
    let graphql_handler = GraphQLHandler::new(GraphQLContext::from_request, Query, Mutation);
    router.any("/graphql", graphql_handler, "graphql");

    let graphiql_handler = GraphiQLHandler::new("/graphql");
    router.any("/", graphiql_handler, "graphiql");

    router.get("/files/music/:id/raw", raw_handler, "raw");

    // Setup Context Middleware
    let mut chain = Chain::new(router);
    chain.link(IronContext::init_middleware(pool));

    // Setup Logging Middleware
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    // Start Server
    println!("Starting Server on {}", host);
    let iron = Iron::new(chain);
    iron.http(host).unwrap();
}
