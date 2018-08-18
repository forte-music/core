extern crate lru_disk_cache;
extern crate rand;
extern crate uuid;

mod files;
mod graphql;
mod stream;
mod streaming;
pub mod temp;
mod transcoder;
mod transcoding;

use forte_core::context;
use forte_core::models::{create_schema, Album, Song};

use server::graphql::{graphiql, graphql, AppState, GraphQLExecutor};
use server::temp::TemporaryFiles;
use server::transcoder::{TranscodeTarget, Transcoder};

use actix::prelude::*;
use actix::System;
use actix_web::http;
use actix_web::server;
use actix_web::App;

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
    let transcoder_addr: Addr<Syn, Transcoder> = transcoder.start();

    server::new(move || {
        App::with_state(AppState::new(
            gql_executor.clone(),
            transcoder_addr.clone(),
            pool.clone(),
        )).resource("/graphql", |r| r.method(http::Method::POST).with2(graphql))
            .resource("/", |r| r.method(http::Method::GET).h(graphiql))
            .resource(&Song::get_mp3_stream_url("{id}"), |r| {
                r.method(http::Method::GET)
                    .h(transcoding::TranscodedSongHandler::new(
                        TranscodeTarget::MP3V0,
                    ))
            })
            .resource(&Song::get_raw_stream_url("{id}"), |r| {
                r.method(http::Method::GET).with2(streaming::song_handler)
            })
            .resource(&Album::get_artwork_url("{id}"), |r| {
                r.method(http::Method::GET)
                    .with2(streaming::artwork_handler)
            })
    }).bind(host)
        .unwrap()
        .start();

    println!("Started Server on {}", host);

    let _ = sys.run();
}
