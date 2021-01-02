use crate::server::graphql::AppState;
use actix_files::NamedFile;
use actix_web::error;
use actix_web::web::{Data, Path};
use forte_core::models::album::Album;
use forte_core::models::song::Song;
use uuid::Uuid;

pub fn convert_diesel_err(err: diesel::result::Error) -> actix_web::Error {
    match err {
        diesel::result::Error::NotFound => error::ErrorNotFound(err),
        _ => error::ErrorInternalServerError(err),
    }
}

pub async fn song_handler(
    state: Data<AppState>,
    Path((song_id,)): Path<(Uuid,)>,
) -> actix_web::Result<NamedFile> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let song = Song::from_id(&context.connection(), song_id.into()).map_err(convert_diesel_err)?;

    Ok(NamedFile::open(song.path.as_path())?)
}

pub async fn artwork_handler(
    state: Data<AppState>,
    Path((album_id,)): Path<(Uuid,)>,
) -> actix_web::Result<NamedFile> {
    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let album =
        Album::from_id(&context.connection(), album_id.into()).map_err(convert_diesel_err)?;
    let artwork_path = album
        .artwork_path
        .ok_or_else(|| error::ErrorNotFound("no artwork"))?;

    Ok(NamedFile::open(artwork_path.as_path())?)
}
