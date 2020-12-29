use super::errors;
use crate::database::song;
use crate::database::song_artist;
use crate::import::album::add_or_get_album;
use crate::import::artist::add_or_get_artist;
use crate::models::*;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::result;
use diesel::Connection;
use std::path::Path;
use taglib2_sys::SongProperties;

/// Checks whether the file at the path is already imported.
pub fn is_imported(path: &Path, conn: &SqliteConnection) -> errors::Result<bool> {
    let does_exist = song::table
        .select(song::id)
        .filter(song::path.eq(PathWrapper::from(path)))
        .first::<UUID>(conn)
        .optional()?;

    Ok(does_exist.is_some())
}

/// Takes information about a song read from tags and adds it to the database.
pub fn add_song(
    path: &Path,
    artwork_directory: &Path,
    props: SongProperties,
    conn: &SqliteConnection,
) -> super::errors::Result<()> {
    let artist: Option<Artist> = props
        .artist
        .as_ref()
        .map_or(Ok(None), |name| add_or_get_artist(name, conn).map(Some))?;

    let album_artist: Option<Artist> = props
        .album_artist
        .as_ref()
        .map_or(Ok(None), |name| add_or_get_artist(name, conn).map(Some))?;

    let (artist, album_artist) = (
        artist
            .as_ref()
            .or_else(|| album_artist.as_ref())
            .ok_or(errors::ErrorKind::NoArtistError)?,
        album_artist
            .as_ref()
            .or_else(|| artist.as_ref())
            .ok_or(errors::ErrorKind::NoArtistError)?,
    );

    let album = add_or_get_album(path, artwork_directory, &props, album_artist.id, conn)?;

    let song_id = UUID::new();
    let song = Song {
        id: song_id,
        name: props.title.ok_or(errors::ErrorKind::NoTitleError)?,
        album_id: album.id,
        track_number: props.track_number as i32,
        disk_number: props.disk_number.map_or(1, |n| n as i32),
        duration: props.duration,
        time_added: Utc::now().naive_utc(),
        play_count: 0,
        last_played: None,
        liked: false,
        path: path.into(),
    };

    conn.transaction::<(), result::Error, _>(|| {
        song.insert_into(song::table).execute(conn)?;

        diesel::insert_into(song_artist::table)
            .values((
                song_artist::song_id.eq(song_id),
                song_artist::artist_id.eq(artist.id),
            ))
            .execute(conn)?;

        Ok(())
    })?;

    Ok(())
}
