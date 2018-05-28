use forte_core::models::song::Song;

use actix_web;
use actix_web::Path;
use actix_web::Result;
use actix_web::State;
use actix_web::error;
use actix_web::fs::NamedFile;

use server::graphql::AppState;

use std::ops::Deref;

use uuid::Uuid;

use diesel;
use forte_core::models::album::Album;

fn convert_diesel_err(err: diesel::result::Error) -> actix_web::Error {
    match err {
        diesel::result::Error::NotFound => error::ErrorNotFound(err),
        _ => error::ErrorInternalServerError(err),
    }
}

pub fn song_handler(state: State<AppState>, song_id: Path<Uuid>) -> Result<NamedFile> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let song = Song::from_id(&context, &song_id.into_inner().into()).map_err(convert_diesel_err)?;

    Ok(NamedFile::open(&song.path.deref())?)
}

pub fn artwork_handler(state: State<AppState>, album_id: Path<Uuid>) -> Result<NamedFile> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let album =
        Album::from_id(&context, &album_id.into_inner().into()).map_err(convert_diesel_err)?;
    let artwork_path = album
        .artwork_path
        .ok_or(error::ErrorNotFound("no artwork"))?;

    Ok(NamedFile::open(&artwork_path.deref())?)
}
