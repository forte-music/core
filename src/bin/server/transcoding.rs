use actix_web;
use actix_web::error;
use actix_web::AsyncResponder;
use actix_web::Path;
use actix_web::State;

use forte_core::models::Song;

use server::graphql::AppState;
use server::stream::RangeStream;
use server::transcoder::{TranscodeMessage, TranscodeTarget};

use diesel;

use uuid::Uuid;

use futures::future;
use futures::Future;

use lru_disk_cache::ReadSeek;

use std::io;
use std::io::SeekFrom;
use std::path::PathBuf;

pub fn handler(
    state: State<AppState>,
    song_id: Path<Uuid>,
) -> actix_web::FutureResponse<RangeStream<Box<ReadSeek>>> {
    let song_id = song_id.into_inner();

    future::done(build_transcode_message(song_id, &state))
        .and_then(move |transcode_msg| {
            state
                .transcoder
                .send(transcode_msg)
                .map_err(error::ErrorInternalServerError)
        })
        .and_then(|result| {
            result.map_err(|e| error::ErrorInternalServerError(e.description().to_string()))
        })
        .and_then(|mut reader| {
            get_size(&mut reader)
                .map(|size| (reader, size))
                .map_err(error::ErrorInternalServerError)
        })
        .map(|(reader, size)| RangeStream::new(reader, size))
        .responder()
}

fn build_transcode_message(
    song_id: Uuid,
    state: &State<AppState>,
) -> actix_web::Result<TranscodeMessage<PathBuf>> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let song = Song::from_id(&context, &song_id.into()).map_err(convert_diesel_err)?;

    let song_path: PathBuf = song.path.into();

    Ok(TranscodeMessage::new(
        song_path,
        song_id.to_string(),
        TranscodeTarget::MP3V0,
    ))
}

fn get_size<R: ReadSeek>(reader: &mut R) -> io::Result<u64> {
    let size = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(0))?;

    Ok(size)
}

fn convert_diesel_err(err: diesel::result::Error) -> actix_web::Error {
    match err {
        diesel::result::Error::NotFound => error::ErrorNotFound(err),
        _ => error::ErrorInternalServerError(err),
    }
}
