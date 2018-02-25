extern crate forte_core;
extern crate redis;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod structs;

use forte_core::actions;
use forte_core::database;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::de::Deserialize;
use redis::Connection;
use forte_core::schema::model::Playlist;
use forte_core::schema::model::PlaylistItem;

fn main() {
    let path = Path::new("./node_modules/@forte-music/schema/fixtures");
    if !path.is_dir() {
        println!(
            "The fixtures can't be found. This command must be run from the fixture_setup \
             directory of the source after running `yarn install`."
        );

        return;
    }

    let pool = database::init_pool().expect("Could not connect to the database");
    let db = pool.get().unwrap();

    load_everything(path.to_str().unwrap(), &db).unwrap();
}

fn load_everything(path: &str, db: &Connection) -> Result<(), Box<Error>> {
    load_artists(path, db)?;
    load_albums(path, db)?;
    load_playlists(path, db)?;
    load_songs(path, db)?;
    load_stats(path, db)?;

    Ok(())
}

fn load_artists(path: &str, db: &Connection) -> Result<(), Box<Error>> {
    let artists = read_artists(path)?;

    for artist_source in artists.artists {
        let album_ids = artist_source.album_ids.clone();
        let id = artist_source.id.clone();

        let artist = artist_source.into();

        actions::add_artist(&artist, db)?;
        actions::add_albums_to_artist(&id, &album_ids, db)?;
    }

    Ok(())
}

fn load_albums(path: &str, db: &Connection) -> Result<(), Box<Error>> {
    let albums = read_albums(path)?;

    for album_source in albums.albums {
        let song_ids = album_source.song_ids.clone();

        let id = album_source.id.clone();
        let album = album_source.into();

        actions::add_album(&album, db)?;
        actions::add_songs_to_album(&id, &song_ids, db)?;
    }

    Ok(())
}

fn load_playlists(path: &str, db: &Connection) -> Result<(), Box<Error>> {
    let playlists = read_playlists(path)?;

    for playlist_source in playlists.playlists {
        let song_ids = playlist_source.song_ids.clone();

        let id = playlist_source.id.clone();
        let playlist: Playlist = playlist_source.into();

        actions::add_playlist(&playlist, db)?;

        let playlist_items: Vec<PlaylistItem> = song_ids
            .into_iter()
            .enumerate()
            .map(|(index, song_id)| PlaylistItem {
                id: format!("{}:{}", id, index),
                song_id,
            })
            .collect();

        for playlist_item in &playlist_items {
            actions::add_playlist_item(playlist_item, db)?;
        }

        let playlist_item_ids: Vec<String> = playlist_items
            .into_iter()
            .map(|playlist_item| playlist_item.id)
            .collect();

        actions::add_playlist_items_to_playlist(&id, &playlist_item_ids, db)?;
    }

    Ok(())
}

fn load_songs(path: &str, db: &Connection) -> Result<(), Box<Error>> {
    let import = read_songs(path)?;

    for song_source in import.songs {
        let id = song_source.id.clone();
        let artist_ids = song_source.artist_ids.clone().unwrap_or_default();

        let song = song_source.into();

        actions::add_song(&song, db)?;
        actions::add_artists_to_song(&id, &artist_ids, db)?;
    }

    Ok(())
}

fn load_stats(path: &str, db: &Connection) -> Result<(), Box<Error>> {
    let import = read_stats(path)?;

    for stats_source in import.stats {
        let stats = stats_source.into();
        actions::add_song_stats(&stats, db)?;
    }

    Ok(())
}

fn read_artists(path: &str) -> Result<structs::ArtistImport, Box<Error>> {
    let mut buffer = String::new();
    read_items(&format!("{}/{}", path, "artists.toml"), &mut buffer)
}

fn read_albums(path: &str) -> Result<structs::AlbumImport, Box<Error>> {
    let mut buffer = String::new();
    read_items(&format!("{}/{}", path, "albums.toml"), &mut buffer)
}

fn read_playlists(path: &str) -> Result<structs::PlaylistImport, Box<Error>> {
    let mut buffer = String::new();
    read_items(&format!("{}/{}", path, "playlists.toml"), &mut buffer)
}

fn read_songs(path: &str) -> Result<structs::SongImport, Box<Error>> {
    let mut buffer = String::new();
    read_items(&format!("{}/{}", path, "songs.toml"), &mut buffer)
}

fn read_stats(path: &str) -> Result<structs::StatsImport, Box<Error>> {
    let mut buffer = String::new();
    read_items(&format!("{}/{}", path, "stats.toml"), &mut buffer)
}

fn read_items<'de, T: Deserialize<'de>>(
    path: &str,
    buffer: &'de mut String,
) -> Result<T, Box<Error>> {
    let mut f = File::open(path)?;
    f.read_to_string(buffer)?;

    let imported: T = toml::from_str(buffer)?;

    Ok(imported)
}
