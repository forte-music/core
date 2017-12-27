use schema::model::*;
use juniper::{ID, FieldResult, FieldError};
use redis::{self, Commands};
use serde::Deserialize;
use serde_redis::RedisDeserialize;

pub trait FromId: Sized {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Self>;
}

fn from_id<'a, T: Deserialize<'a>>(key: &str, db: &redis::Connection) -> FieldResult<T> {
    // Deserialize the struct
    let result: redis::Value = db.hgetall(key)?;

    if let redis::Value::Bulk(ref data) = result {
        if data.len() == 0 {
            return Err(FieldError::from(format!("{} does not exist", key)));
        }
    }
    else {
        return Err(FieldError::from("Database error"));
    }

    Ok(result.deserialize()?)
}

impl Query {
    pub fn album(db: &redis::Connection, id: ID) -> FieldResult<Album> {
        Album::from_id(&id, db)
    }

    pub fn artist(db: &redis::Connection, id: ID) -> FieldResult<Artist> {
        Artist::from_id(&id, db)
    }

    pub fn song(db: &redis::Connection, id: ID) -> FieldResult<Song> {
        Song::from_id(&id, db)
    }

    pub fn playlist(db: &redis::Connection, id: ID) -> FieldResult<Playlist> {
        Playlist::from_id(&id, db)
    }
}

impl FromId for Album {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Album> {
        from_id(&Album::key(id), db)
    }
}

impl Album {
    fn key(id: &str) -> String {
        format!("album:{}", id)
    }

    pub fn artist(&self, db: &redis::Connection) -> FieldResult<Artist> {
        Artist::from_id(&self.artist_id, db)
    }

    pub fn songs(&self, db: &redis::Connection) -> FieldResult<Vec<Song>> {
        let key = format!("{}:songs", Album::key(&self.id));
        let song_ids: Vec<String> = db.smembers(key)?;
        let mut songs: Vec<Song> = Vec::with_capacity(song_ids.len());

        for id in song_ids {
            songs.push(Song::from_id(&id, db)?);
        }

        Ok(songs)
    }
}

impl FromId for Artist {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Artist> {
        from_id(&Artist::key(id), db)
    }
}

impl Artist {
    fn key(id: &str) -> String {
        format!("artist:{}", id)
    }
}

impl FromId for Song {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Song> {
        from_id(&Song::key(id), db)
    }
}

impl Song {
    fn key(id: &str) -> String {
        format!("song:{}", id)
    }

    pub fn album(&self, db: &redis::Connection) -> FieldResult<Album> {
        Album::from_id(&self.album_id, db)
    }
}

impl FromId for SongUserStats {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<SongUserStats> {
        Ok(SongUserStats { id: id.to_owned(), .. SongUserStats::default() })
    }
}

impl FromId for Playlist {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Playlist> {
        Ok(Playlist { id: id.to_owned(), .. Playlist::default() })
    }
}
