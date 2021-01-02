mod graphql;
mod streaming;
pub mod temp;
mod transcoder;
mod transcoding;

#[cfg(feature = "embed_web")]
mod web_interface;

use crate::server::graphql::{graphiql, graphql as graphql_handler, AppState};
use crate::server::temp::TemporaryFiles;
use crate::server::transcoder::Transcoder;
use crate::server::transcoding::transcode_handler;
use actix_web::rt::System;
use actix_web::{web, App, HttpServer};
use forte_core::context;
use forte_core::models::{create_schema, Album, Song};
use lru_disk_cache::LruDiskCache;

#[cfg(feature = "embed_web")]
use web_interface::register_web_interface_handler;

#[cfg(not(feature = "embed_web"))]
use actix_web::web::ServiceConfig;
/// Do nothing when the embedded web interface is not enabled
#[cfg(not(feature = "embed_web"))]
fn register_web_interface_handler(_config: &mut ServiceConfig) {}

pub fn serve(
    pool: context::Pool,
    host: &str,
    transcode_cache: LruDiskCache,
    temp_files: TemporaryFiles,
) -> std::io::Result<()> {
    let mut sys = System::new("forte");
    let transcoder = Transcoder::new(transcode_cache, temp_files);

    let server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                schema: create_schema(),
                transcoder: transcoder.clone(),
                connection_pool: pool.clone(),
            })
            .service(graphql_handler)
            .service(graphiql)
            .route(
                &Song::get_raw_stream_url("{id}"),
                web::get().to(streaming::song_handler),
            )
            .route(
                &Album::get_artwork_url("{id}"),
                web::get().to(streaming::artwork_handler),
            )
            .service(transcode_handler)
            .configure(register_web_interface_handler)
    })
    .bind(host)
    .unwrap();

    println!("Starting Server on {}", host);

    sys.block_on(server.run())
}
