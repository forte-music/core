use chrono::prelude::*;
use database::artist;
use diesel::prelude::*;
use models::*;

pub fn add_or_get_artist(name: &str, conn: &SqliteConnection) -> QueryResult<Artist> {
    let artist: Option<Artist> = artist::table
        .filter(artist::name.eq(name))
        .first(conn)
        .optional()?;

    if let Some(artist) = artist {
        return Ok(artist);
    }

    let artist = Artist {
        id: UUID::new(),
        name: name.to_string(),
        time_added: Utc::now().naive_utc(),
        last_played: None,
    };

    artist.clone().insert_into(artist::table).execute(conn)?;

    Ok(artist)
}
