use actix_web;
use actix_web::Path;
use actix_web::State;
use forte_core::models::Song;
use server::graphql::AppState;

use actix_web::error;
use diesel;
use server::files::FileStream;
use std::ops::Deref;
use uuid::Uuid;

fn convert_diesel_err(err: diesel::result::Error) -> actix_web::Error {
    match err {
        diesel::result::Error::NotFound => error::ErrorNotFound(err),
        _ => error::ErrorInternalServerError(err),
    }
}

pub fn handler(state: State<AppState>, song_id: Path<Uuid>) -> actix_web::Result<FileStream> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let song = Song::from_id(&context, &song_id.into_inner().into()).map_err(convert_diesel_err)?;
    let song_path = song.path.deref();

    Ok(FileStream::open(song_path)?)
}

// TODO: Implement FFMPEG CLI Reader
