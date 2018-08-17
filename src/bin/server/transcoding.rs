use actix_web;
use actix_web::error;
use actix_web::AsyncResponder;
use actix_web::Path;
use actix_web::State;

use forte_core::models::Song;

use server::files::FileStream;
use server::graphql::AppState;
use server::transcoder::{Transcode, TranscodeTarget};

use diesel;

use uuid::Uuid;

use std::ops::Deref;

use futures;
use futures::Future;

use lru_disk_cache::ReadSeek;
use server::stream::RangeStream;
use std::io::SeekFrom;
use std::path::PathBuf;

fn convert_diesel_err(err: diesel::result::Error) -> actix_web::Error {
    match err {
        diesel::result::Error::NotFound => error::ErrorNotFound(err),
        _ => error::ErrorInternalServerError(err),
    }
}

pub fn handler(
    state: State<AppState>,
    song_id: Path<Uuid>,
) -> actix_web::FutureResponse<RangeStream<Box<ReadSeek>>> {
    let song_id = song_id.into_inner();

    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)
        .expect("// TODO: Remove");

    let song = Song::from_id(&context, &song_id.into())
        .map_err(convert_diesel_err)
        .expect("// TODO: Remove");

    let song_path: PathBuf = song.path.into();

    state
        .transcoder
        .send(Transcode::new(
            song_path,
            song_id.to_string(),
            TranscodeTarget::MP3V0,
        ))
        .map(|f| {
            let mut reader = f.expect("// TODO: Remove");

            let size = reader.seek(SeekFrom::End(0)).expect("// TODO: Remove");
            reader.seek(SeekFrom::Start(0)).expect("// TODO: Remove");

            RangeStream::new(reader, size)
        })
        .map_err(|e| error::ErrorInternalServerError(e))
        .responder()
}
