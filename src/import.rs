use taglib2_sys::SongProperties;

use chrono::Utc;

use database::album;
use database::artist;
use database::song_artist;

use diesel;
use diesel::Connection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result;

use models::*;
use std::path::Path;

pub mod errors {
    error_chain! {
        foreign_links {
            Diesel(::diesel::result::Error);
        }

        errors {
            NoArtistError {
                description("either the tag's album artist or artist needs to be set, neither is")
            }

            NoAlbumError {
                description("the album name wasn't specified in the tag")
            }

            NoTitleError {
                description("the title wasn't specified in the tag")
            }
        }
    }
}

/// Takes information about a song read from tags and adds it to the database.
pub fn add_song(path: &Path, props: SongProperties, conn: &SqliteConnection) -> errors::Result<()> {
    let artist: Option<Artist> = props
        .artist
        .map_or(Ok(None), |name| add_or_get_artist(name, conn).map(Some))?;

    let album_artist: Option<Artist> = props
        .album_artist
        .map_or(Ok(None), |name| add_or_get_artist(name, conn).map(Some))?;

    let (artist, album_artist) = (
        artist
            .clone()
            .or(album_artist.clone())
            .ok_or(errors::ErrorKind::NoArtistError)?,
        album_artist
            .or(artist)
            .ok_or(errors::ErrorKind::NoArtistError)?,
    );

    let album_name = props.album.ok_or(errors::ErrorKind::NoAlbumError)?;
    let album = add_or_get_album(album_name, album_artist.id, props.year, conn)?;

    let song_id = UUID::new();
    let song = Song {
        id: song_id,
        name: props.title.ok_or(errors::ErrorKind::NoTitleError)?,
        album_id: album.id,
        track_number: props.track_number as i32,
        duration: props.duration,
        time_added: Utc::now().naive_utc(),
        play_count: 0,
        last_played: None,
        liked: false,
        path: path.into(),
    };

    conn.transaction::<(), result::Error, _>(|| {
        song.insert_into(Song::table()).execute(conn)?;

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

fn add_or_get_artist(name: String, conn: &SqliteConnection) -> Result<Artist, result::Error> {
    let artist: Option<Artist> = artist::table
        .filter(artist::name.eq(name.as_str()))
        .first(conn)
        .optional()?;

    if let Some(artist) = artist {
        return Ok(artist);
    }

    let artist = Artist {
        id: UUID::new(),
        name,
        time_added: Utc::now().naive_utc(),
        last_played: None,
    };

    artist.clone().insert_into(Artist::table()).execute(conn)?;

    Ok(artist)
}

fn add_or_get_album(
    name: String,
    artist_id: UUID,
    release_year: Option<u32>,
    conn: &SqliteConnection,
) -> Result<Album, result::Error> {
    let album: Option<Album> = album::table
        .filter(album::name.eq(name.as_str()))
        .filter(album::artist_id.eq(artist_id))
        .first(conn)
        .optional()?;

    if let Some(album) = album {
        return Ok(album);
    }

    let album = Album {
        id: UUID::new(),
        artwork_url: None,
        name,
        artist_id,
        release_year: release_year.map(|year| year as i32),
        time_added: Utc::now().naive_utc(),
        last_played: None,
    };

    album.clone().insert_into(Album::table()).execute(conn)?;

    Ok(album)
}
