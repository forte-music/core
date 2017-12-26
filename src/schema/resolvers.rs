use std::ops::Deref;
use schema::model::*;
use juniper::{ID, FieldResult, FieldError};
use redis::{self, Commands};
use serde_redis::RedisDeserialize;

pub trait FromId: Sized {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Self>;
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

    pub fn generic_connection<T>(db: &redis::Connection, limit: i32, cursor: Option<String>) -> Connection<T> {
        Connection {
            count: 0,
            edges: vec![]
        }
    }
}

impl FromId for Album {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Album> {
        let key = Album::key(id);

        // Deserialize the album
        let result = db.hgetall::<&str, redis::Value>(&key)?;

        println!("{:?}", result);

        if let redis::Value::Bulk(ref data) = result {
            if data.len() == 0 {
                return Err(FieldError::from("Album does not exist"));
            }
        }
        else {
            return Err(FieldError::from("Database error"));
        }

        Ok(result.deserialize()?)
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
        let key = Artist::key(id);

        // Deserialize the artist
        let result = db.hgetall::<&str, redis::Value>(&key)?;

        println!("{:?}", result);

        if let redis::Value::Bulk(ref data) = result {
            if data.len() == 0 {
                return Err(FieldError::from("Artist does not exist"));
            }
        }
        else {
            return Err(FieldError::from("Database error"));
        }

        Ok(result.deserialize()?)
    }
}

impl Artist {
    fn key(id: &str) -> String {
        format!("artist:{}", id)
    }
}

impl FromId for Song {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Song> {
        let key = Song::key(id);

        // Deserialize the song
        let result = db.hgetall::<&str, redis::Value>(&key)?;

        println!("{:?}", result);

        if let redis::Value::Bulk(ref data) = result {
            if data.len() == 0 {
                return Err(FieldError::from("Song does not exist"));
            }
        }
        else {
            return Err(FieldError::from("Database error"));
        }

        Ok(result.deserialize()?)
    }
}

impl Song {
    fn key(id: &str) -> String {
        format!("song:{}", id)
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
