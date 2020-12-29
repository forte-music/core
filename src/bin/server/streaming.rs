use crate::server::files::FileStream;
use crate::server::graphql::AppState;
use actix_web::error;
use actix_web::Path;
use actix_web::Result;
use actix_web::State;
use forte_core::models::album::Album;
use forte_core::models::song::Song;
use std::ops::Deref;
use uuid::Uuid;

fn convert_diesel_err(err: diesel::result::Error) -> actix_web::Error {
    match err {
        diesel::result::Error::NotFound => error::ErrorNotFound(err),
        _ => error::ErrorInternalServerError(err),
    }
}

pub fn song_handler(params: (State<AppState>, Path<Uuid>)) -> Result<FileStream> {
    let state = params.0;
    let song_id = params.1;

    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let song = Song::from_id(&context, &song_id.into_inner().into()).map_err(convert_diesel_err)?;

    Ok(FileStream::open(&song.path.deref())?)
}

pub fn artwork_handler(params: (State<AppState>, Path<Uuid>)) -> Result<FileStream> {
    let state = params.0;
    let album_id = params.1;

    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let album =
        Album::from_id(&context, &album_id.into_inner().into()).map_err(convert_diesel_err)?;
    let artwork_path = album
        .artwork_path
        .ok_or_else(|| error::ErrorNotFound("no artwork"))?;

    Ok(FileStream::open(&artwork_path.deref())?)
}
