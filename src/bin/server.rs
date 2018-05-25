extern crate iron;
extern crate juniper_iron;
extern crate logger;
extern crate persistent;
extern crate router;
extern crate uuid;

use diesel::result;

use forte_core::context;
use forte_core::context::{GraphQLContext, IronContext};
use forte_core::models::{Album, Mutation, Query, Song, UUID};

use self::iron::IronError;
use self::iron::IronResult;
use self::iron::Request;
use self::iron::Response;
use self::iron::status;
use self::iron::{Chain, Iron};

use self::juniper_iron::{GraphQLHandler, GraphiQLHandler};
use self::logger::Logger;
use self::router::Router;

use diesel::result::QueryResult;
use std::ops::Deref;
use std::path::Path;

mod errors {
    error_chain! {
        errors {
            EmptyOption {
                description("the option was empty when it was expected to be non empty")
            }
        }
    }
}

fn convert_parse_error(r: Result<UUID, uuid::ParseError>) -> IronResult<UUID> {
    r.map_err(|err| IronError::new(err, ("invalid uuid", status::BadRequest)))
}

fn convert_query_error<T>(r: QueryResult<T>) -> IronResult<T> {
    r.map_err(|err| match err {
        result::Error::NotFound => IronError::new(err, status::NotFound),
        _ => IronError::new(err, status::InternalServerError),
    })
}

fn convert_option<T>(r: Option<T>, status: status::Status) -> IronResult<T> {
    r.ok_or(IronError::new(
        errors::Error::from(errors::ErrorKind::EmptyOption),
        status,
    ))
}

fn song_stream_handler(req: &mut Request) -> IronResult<Response> {
    let ctx = GraphQLContext::from_request(req);
    let ref id = convert_option(
        req.extensions.get::<Router>().unwrap().find("id"),
        status::BadRequest,
    )?;
    let uuid = convert_parse_error(UUID::parse_str(id))?;
    let song = convert_query_error(Song::from_id(&ctx, &uuid))?;
    let path: &Path = song.path.deref();

    Ok(Response::with((status::Ok, path)))
}

fn artwork_stream_handler(req: &mut Request) -> IronResult<Response> {
    let ctx = GraphQLContext::from_request(req);
    let ref id = convert_option(
        req.extensions.get::<Router>().unwrap().find("id"),
        status::BadRequest,
    )?;
    let uuid = convert_parse_error(UUID::parse_str(id))?;

    let album = convert_query_error(Album::from_id(&ctx, &uuid))?;
    let artwork_path_wrapper = convert_option(album.artwork_path, status::NotFound)?;
    let artwork_path: &Path = artwork_path_wrapper.deref();

    Ok(Response::with((status::Ok, artwork_path)))
}

pub fn serve(pool: context::Pool, host: String) {
    let mut router = Router::new();

    // Register Routes
    let graphql_handler = GraphQLHandler::new(GraphQLContext::from_request, Query, Mutation);
    router.any("/graphql", graphql_handler, "graphql");

    let graphiql_handler = GraphiQLHandler::new("/graphql");
    router.any("/", graphiql_handler, "graphiql");

    router.get(
        Song::get_stream_url(":id"),
        song_stream_handler,
        "audio_stream",
    );

    router.get(
        Album::get_artwork_url(":id"),
        artwork_stream_handler,
        "artwork",
    );

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
