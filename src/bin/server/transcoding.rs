use crate::server::graphql::AppState;
use crate::server::streaming::convert_diesel_err;
use crate::server::transcoder::TranscodeRequest;
use actix_files::NamedFile;
use actix_web::error;
use actix_web::get;
use actix_web::web::{Data, Path};
use forte_core::models::Song;
use uuid::Uuid;

#[get("/files/music/{id}/{target:.+}")]
pub async fn transcode_handler(
    state: Data<AppState>,
    Path((song_id, target_str)): Path<(Uuid, String)>,
) -> actix_web::Result<NamedFile> {
    let target = target_str.parse().map_err(error::ErrorNotFound)?;

    let context = state
        .build_context()
        .map_err(error::ErrorInternalServerError)?;

    let song = Song::from_id(&context.connection(), song_id.into()).map_err(convert_diesel_err)?;
    let transcode_msg = TranscodeRequest::new(song.path.to_path_buf(), song.id.to_string(), target);
    let file = state
        .transcoder
        .get_transcoded_file(&transcode_msg)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(NamedFile::from_file(file, target.get_filename(&song.name))?)
}
