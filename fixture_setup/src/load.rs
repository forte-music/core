use forte_core::context;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use dotenv;
use toml;

use diesel::associations::HasTable;
use diesel::prelude::*;

use errors::*;
use forte_core::models::*;
use source_models::*;

pub fn load() -> Result<()> {
    dotenv::dotenv().ok();

    let path = Path::new("./node_modules/@forte-music/schema/fixtures");
    if !path.is_dir() {
        return Err(
            "The fixtures can't be found. This command must be run from the fixture_setup \
             directory of the source after running `yarn install`."
                .into(),
        );
    }

    let pool = context::init_pool().map_err(|err| Error::from(err.description()))?;
    let db = pool.get()?;

    load_from_folder(&path, &db)?;

    Ok(())
}

fn load_from_folder(path: &Path, conn: &SqliteConnection) -> Result<()> {
    let files = path.read_dir()?;
    for file in files {
        let path = file?.path();

        if path.is_dir() || !path.extension().map_or(false, |ext| ext == "toml") {
            continue;
        }

        load_from_file(&path, conn)?;
    }

    Ok(())
}

fn load_from_file(path: &Path, conn: &SqliteConnection) -> Result<()> {
    let mut buffer = String::new();
    let imported: Import = read_items(path, &mut buffer)?;

    if let Some(albums) = imported.albums {
        add_all_albums(albums, conn)?;
    };

    if let Some(artists) = imported.artists {
        add_all_artists(artists, conn)?;
    };

    if let Some(playlists) = imported.playlists {
        add_all_playlists(playlists, conn)?;
    };

    if let Some(songs) = imported.songs {
        add_all_songs(songs, conn)?;
    };

    Ok(())
}

fn add_all_albums(things: Vec<AlbumSource>, conn: &SqliteConnection) -> Result<()> {
    for thing in things {
        let thing: Album = thing.into();
        thing.insert_into(Album::table()).execute(conn)?;
    }

    Ok(())
}

fn add_all_artists(things: Vec<ArtistSource>, conn: &SqliteConnection) -> Result<()> {
    for thing in things {
        let thing: Artist = thing.into();
        thing.insert_into(Artist::table()).execute(conn)?;
    }

    Ok(())
}

fn add_all_playlists(things: Vec<PlaylistSource>, conn: &SqliteConnection) -> Result<()> {
    for thing in things {
        let thing: Playlist = thing.into();
        thing.insert_into(Playlist::table()).execute(conn)?;
    }

    Ok(())
}

fn add_all_songs(things: Vec<SongSource>, conn: &SqliteConnection) -> Result<()> {
    for thing in things {
        let thing: Song = thing.into();
        thing.insert_into(Song::table()).execute(conn)?;
    }

    Ok(())
}

fn read_items<'de, T: Deserialize<'de>>(path: &Path, mut buffer: &'de mut String) -> Result<T> {
    let mut f = File::open(path)?;
    f.read_to_string(&mut buffer)?;

    let imported: T = toml::from_str(buffer)?;

    Ok(imported)
}
