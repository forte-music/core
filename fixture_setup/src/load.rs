use diesel;
use forte_core::context;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use dotenv;
use toml;

use diesel::associations::HasTable;
use diesel::prelude::*;

use forte_core::database::song_artist;

use errors::*;
use forte_core::models::*;
use source_models::*;

/// Load the test data into the database.
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

    let pool = context::init_pool(&env::var("DATABASE_URL")?)?;
    let db = pool.get()?;

    load_from_folder(&path, &db)?;

    Ok(())
}

/// Read test data from a folder.
/// The test files must be in TOML format and end in `.toml`.
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

/// Load test data from a TOML file
fn load_from_file(path: &Path, conn: &SqliteConnection) -> Result<()> {
    let mut buffer = String::new();
    let imported: Import = read_items(path, &mut buffer)?;

    if let Some(albums) = imported.albums {
        add_all_albums(albums, conn)?;
    };

    if let Some(artists) = imported.artists {
        add_all_artists(artists, conn)?;
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

fn add_all_songs(things: Vec<SongSource>, conn: &SqliteConnection) -> Result<()> {
    for song_source in things {
        let artist_ids = song_source.artist_ids.clone().unwrap_or(vec![]);
        let song: Song = song_source.into();
        let song_id = song.id.clone();

        song.insert_into(Song::table()).execute(conn)?;

        let records: Vec<_> = artist_ids
            .into_iter()
            .map(|id| id.into())
            .map(|artist_id: UUID| {
                (
                    song_artist::song_id.eq(song_id),
                    song_artist::artist_id.eq(artist_id),
                )
            })
            .collect();

        diesel::insert_into(song_artist::table)
            .values(records)
            .execute(conn)?;
    }

    Ok(())
}

/// Parse a TOML file into a variable of type `T`
fn read_items<'de, T: Deserialize<'de>>(path: &Path, mut buffer: &'de mut String) -> Result<T> {
    let mut f = File::open(path)?;
    f.read_to_string(&mut buffer)?;

    let imported: T = toml::from_str(buffer)?;

    Ok(imported)
}
