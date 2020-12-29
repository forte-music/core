mod files;
mod graphql;
mod stream;
mod streaming;
pub mod temp;
mod transcoder;
mod transcoding;

#[cfg(feature = "embed_web")]
mod web;

use crate::server::graphql::{graphiql, graphql, AppState, GraphQLExecutor};
use crate::server::temp::TemporaryFiles;
use crate::server::transcoder::{TranscodeTarget, Transcoder};
use crate::server::transcoding::TranscodedHandlerAppExt;
use actix::prelude::*;
use actix::System;
use actix_web::http;
use actix_web::server;
use actix_web::App;
use forte_core::context;
use forte_core::models::{create_schema, Album, Song};
use lru_disk_cache::LruDiskCache;
use std::sync::Arc;

pub fn serve(
    pool: context::Pool,
    host: &str,
    transcode_cache: LruDiskCache,
    temp_files: TemporaryFiles,
) {
    let sys = System::new("forte");

    let schema = Arc::new(create_schema());
    let gql_executor = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    let transcoder = Transcoder::new(transcode_cache, temp_files);
    let transcoder_addr: Addr<Transcoder> = transcoder.start();

    server::new(move || {
        App::with_state(AppState::new(
            gql_executor.clone(),
            transcoder_addr.clone(),
            pool.clone(),
        ))
        .resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
        .resource("/graphiql", |r| r.method(http::Method::GET).f(graphiql))
        .register_transcode_handler(TranscodeTarget::MP3V0)
        .register_transcode_handler(TranscodeTarget::AACV5)
        .resource(&Song::get_raw_stream_url("{id}"), |r| {
            r.method(http::Method::GET).with(streaming::song_handler)
        })
        .resource(&Album::get_artwork_url("{id}"), |r| {
            r.method(http::Method::GET).with(streaming::artwork_handler)
        })
        .register_web_interface_handler()
    })
    .bind(host)
    .unwrap()
    .start();

    println!("Started Server on {}", host);

    let _ = sys.run();
}

pub trait WebHandlerAppExt {
    /// Register the web interface handlers, if the embedded web interface is enabled
    fn register_web_interface_handler(self) -> Self;
}

/// Do nothing when the embedded web interface is not enabled
#[cfg(not(feature = "embed_web"))]
impl WebHandlerAppExt for App<AppState> {
    fn register_web_interface_handler(self) -> Self {
        self
    }
}
