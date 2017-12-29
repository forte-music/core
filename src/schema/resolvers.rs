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

fn read_vec_from_db<T: FromId>(key: &str, db: &redis::Connection) -> FieldResult<Vec<T>> {
    let ids: Vec<String> = db.smembers(key)?;
    let mut items: Vec<T> = Vec::with_capacity(ids.len());

    for id in ids {
        items.push(T::from_id(&id, db)?);
    }

    Ok(items)
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
        read_vec_from_db(&format!("{}:songs", Album::key(&self.id)), db)
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

    pub fn albums(&self, db: &redis::Connection) -> FieldResult<Vec<Album>> {
        read_vec_from_db(&format!("{}:albums", Artist::key(&self.id)), db)
    }

    pub fn featured(&self, db: &redis::Connection) -> FieldResult<Vec<Album>> {
        read_vec_from_db(&format!("{}:featured", Artist::key(&self.id)), db)
    }

    pub fn singles(&self, db: &redis::Connection) -> FieldResult<Vec<Album>> {
        read_vec_from_db(&format!("{}:singles", Artist::key(&self.id)), db)
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

    pub fn artists(&self, db: &redis::Connection) -> FieldResult<Vec<Artist>> {
        read_vec_from_db(&format!("{}:artists", Song::key(&self.id)), db)
    }

    pub fn stats(&self, db: &redis::Connection) -> FieldResult<SongUserStats> {
        SongUserStats::from_id(&self.stat_id, db)
    }
}

impl FromId for SongUserStats {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<SongUserStats> {
        from_id(&SongUserStats::key(id), db)
    }
}

impl SongUserStats {
    fn key(id: &str) -> String {
        format!("stat:{}", id)
    }
}

impl FromId for Playlist {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Playlist> {
        Ok(Playlist { id: id.to_owned(), .. Playlist::default() })
    }
}
