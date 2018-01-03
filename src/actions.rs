use schema::model::*;
use redis::{Connection, Commands, RedisResult};

pub fn add_album(album: Album, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(Album::key(&album.id), &[
        ("id", &album.id),
        ("artwork_url", &album.artwork_url.unwrap_or(String::new())),
        ("name", &album.name),
        ("artist_id", &album.artist_id),
        ("release_year", &album.release_year.to_string())
    ])?;

    Ok(())
}