use schema::model::*;
use schema::binding::*;
use juniper::{ID, FieldResult, FieldError};
use redis::{self, Commands};
use serde::Deserialize;
use serde_redis::RedisDeserialize;

pub trait FromId: Sized {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Self>;
}

impl<'a, T: Keyed + Deserialize<'a>> FromId for T {
    fn from_id(id: &str, db: &redis::Connection) -> FieldResult<Self> {
        from_id(&T::key(id), db)
    }
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

fn read_members_from_db<T: FromId>(key: &str, db: &redis::Connection) -> FieldResult<Connection<T>> {
    let items: Vec<Edge<T>> = redis::cmd("SMEMBERS").arg(key)
        .iter::<String>(db)?
        .map(|item| {
            let node = T::from_id(&item, db).unwrap();
            Edge {
                cursor: item,
                node
            }
        })
        .collect();

    Ok(Connection {
        count: items.len(),
        edges: items
    })
}

impl Query {
    pub fn album(db: &redis::Connection, id: ID) -> FieldResult<Album> {
        Album::from_id(&id, db)
    }

    pub fn albums(db: &redis::Connection) -> FieldResult<Connection<Album>> {
        read_members_from_db::<Album>("albums", db)
    }

    pub fn artist(db: &redis::Connection, id: ID) -> FieldResult<Artist> {
        Artist::from_id(&id, db)
    }

    pub fn artists(db: &redis::Connection) -> FieldResult<Connection<Artist>> {
        read_members_from_db::<Artist>("artists", db)
    }

    pub fn song(db: &redis::Connection, id: ID) -> FieldResult<Song> {
        Song::from_id(&id, db)
    }

    pub fn songs(db: &redis::Connection) -> FieldResult<Connection<Song>> {
        read_members_from_db::<Song>("songs", db)
    }

    pub fn playlist(db: &redis::Connection, id: ID) -> FieldResult<Playlist> {
        Playlist::from_id(&id, db)
    }

    pub fn playlists(db: &redis::Connection) -> FieldResult<Connection<Playlist>> {
        read_members_from_db::<Playlist>("playlists", db)
    }
}

impl Album {
    fn songs_key(&self) -> String {
        format!("{}:songs", Album::key(&self.id))
    }

    pub fn artist(&self, db: &redis::Connection) -> FieldResult<Artist> {
        Artist::from_id(&self.artist_id, db)
    }

    pub fn songs(&self, db: &redis::Connection) -> FieldResult<Vec<Song>> {
        read_vec_from_db(&self.songs_key(), db)
    }

    pub fn duration(&self, db: &redis::Connection) -> FieldResult<i32> {
        Ok(
            redis::cmd("SORT")
                .arg(self.songs_key())
                .arg("BY").arg("song:*")
                .arg("GET").arg("song:*->duration")
                .iter::<String>(db)?
                .map(|duration| duration.parse::<i32>().unwrap())
                .sum::<i32>()
        )
    }
}

impl Artist {
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

impl Song {
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

impl Playlist {
    fn items_key(&self) -> String {
        format!("{}:items", Playlist::key(&self.id))
    }

    pub fn items(&self, query: ConnectionQuery, db: &redis::Connection) -> FieldResult<Connection<PlaylistItem>> {
        let items: Vec<Edge<PlaylistItem>> = redis::cmd("LRANGE")
            .arg(self.items_key()).arg(0).arg(-1)
            .iter::<String>(db)?
            .map(|item| {
                let node = PlaylistItem::from_id(&item, db).unwrap();
                Edge {
                    cursor: item,
                    node
                }
            })
            .collect();

        Ok(Connection {
            count: items.len(),
            edges: items
        })
    }
}

impl PlaylistItem {
    pub fn song(&self, db: &redis::Connection) -> FieldResult<Song> {
        Song::from_id(&self.song_id, db)
    }
}
