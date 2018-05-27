use taglib2_sys::SongProperties;

use std::path::Path;
use std::path::PathBuf;

use chrono::prelude::*;
use diesel::prelude::*;
use models::*;

use super::artwork;
use super::errors;
use database::album;

pub fn add_or_get_album(
    path: &Path,
    artwork_dir: &Path,
    props: &SongProperties,
    name: &str,
    artist_id: UUID,
    release_year: Option<u32>,
    conn: &SqliteConnection,
) -> errors::Result<Album> {
    let album: Option<Album> = album::table
        .filter(album::name.eq(name))
        .filter(album::artist_id.eq(artist_id))
        .first(conn)
        .optional()?;

    if let Some(album) = album {
        return Ok(album);
    }

    let id = UUID::new();

    let artwork_path: Option<PathBuf> =
        artwork::get_best_artwork_path(path, artwork_dir, &id.to_string(), props)?;

    let album = Album {
        id,
        artwork_path: artwork_path.map(|p| p.into()),
        name: name.to_string(),
        artist_id,
        release_year: release_year.map(|year| year as i32),
        time_added: Utc::now().naive_utc(),
        last_played: None,
    };

    album.clone().insert_into(album::table).execute(conn)?;

    Ok(album)
}
