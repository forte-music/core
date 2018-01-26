use schema::model::*;
use schema::binding::*;
use juniper::FieldResult;
use redis::{self, Commands};
use serde::Deserialize;
use serde_redis::RedisDeserialize;
use chrono;

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
        if data.is_empty() {
            return Err(format!("{} does not exist", key).into());
        }
    } else {
        return Err("Database error".into());
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

fn read_members_from_db<T: FromId>(
    key: &str,
    db: &redis::Connection,
) -> FieldResult<Connection<T>> {
    let items: Vec<Edge<T>> = redis::cmd("SMEMBERS")
        .arg(key)
        .iter::<String>(db)?
        .map(|item| {
            let node = T::from_id(&item, db).unwrap();
            Edge { cursor: item, node }
        })
        .collect();

    Ok(Connection {
        count: items.len(),
        edges: items,
    })
}

impl Query {
    pub fn album(db: &redis::Connection, id: &str) -> FieldResult<Album> {
        Album::from_id(id, db)
    }

    pub fn albums(db: &redis::Connection) -> FieldResult<Connection<Album>> {
        read_members_from_db::<Album>("albums", db)
    }

    pub fn artist(db: &redis::Connection, id: &str) -> FieldResult<Artist> {
        Artist::from_id(id, db)
    }

    pub fn artists(db: &redis::Connection) -> FieldResult<Connection<Artist>> {
        read_members_from_db::<Artist>("artists", db)
    }

    pub fn song(db: &redis::Connection, id: &str) -> FieldResult<Song> {
        Song::from_id(id, db)
    }

    pub fn songs(db: &redis::Connection) -> FieldResult<Connection<Song>> {
        read_members_from_db::<Song>("songs", db)
    }

    pub fn playlist(db: &redis::Connection, id: &str) -> FieldResult<Playlist> {
        Playlist::from_id(id, db)
    }

    pub fn playlists(db: &redis::Connection) -> FieldResult<Connection<Playlist>> {
        read_members_from_db::<Playlist>("playlists", db)
    }
}

impl Mutation {
    pub fn play_song(db: &redis::Connection, id: &str) -> FieldResult<SongUserStats> {
        let song_key = Song::key(id);

        if !db.exists::<_, bool>(&song_key)? {
            return Err(format!("{} does not exist", song_key).into());
        }

        let stat_id: String = db.hget(&song_key, "stat_id")?;
        let stat_key = SongUserStats::key(&stat_id);

        db.hincr::<_, _, _, ()>(&stat_key, "play_count", 1)?;
        let now = chrono::Utc::now().timestamp();
        db.hset::<_, _, _, ()>(&stat_key, "last_played", now)?;

        SongUserStats::from_id(&stat_id, db)
    }

    pub fn toggle_like(db: &redis::Connection, id: &str) -> FieldResult<SongUserStats> {
        let song_key = Song::key(id);

        if !db.exists::<_, bool>(&song_key)? {
            return Err(format!("{} does not exist", song_key).into());
        }

        let stat_id: String = db.hget(&song_key, "stat_id")?;
        let stat_key = SongUserStats::key(&stat_id);

        let liked = db.hget::<_, _, String>(&stat_key, "liked")? == "true";
        db.hset::<_, _, _, ()>(&stat_key, "liked", if liked { "false" } else { "true" })?;

        SongUserStats::from_id(&stat_id, db)
    }
}

impl Album {
    pub fn songs_key(id: &str) -> String {
        format!("{}:songs", Album::key(id))
    }

    pub fn artist(&self, db: &redis::Connection) -> FieldResult<Artist> {
        Artist::from_id(&self.artist_id, db)
    }

    pub fn songs(&self, db: &redis::Connection) -> FieldResult<Vec<Song>> {
        let key = Album::songs_key(&self.id);
        let ids: Vec<String> = redis::cmd("SORT")
            .arg(&key)
            .arg("BY").arg("song:*->track_number")
            .arg("GET").arg("song:*->id")
            .query(db)?;
        let mut items: Vec<Song> = Vec::with_capacity(ids.len());

        for id in ids {
            items.push(Song::from_id(&id, db)?);
        }

        Ok(items)
    }

    pub fn duration(&self, db: &redis::Connection) -> FieldResult<i32> {
        Ok(redis::cmd("SORT")
            .arg(Album::songs_key(&self.id))
            .arg("BY")
            .arg("song:*")
            .arg("GET")
            .arg("song:*->duration")
            .iter::<String>(db)?
            .map(|duration| duration.parse::<i32>().unwrap())
            .sum::<i32>())
    }
}

impl Artist {
    pub fn albums_key(id: &str) -> String {
        format!("{}:albums", Artist::key(id))
    }

    pub fn albums(&self, db: &redis::Connection) -> FieldResult<Vec<Album>> {
        read_vec_from_db(&Artist::albums_key(&self.id), db)
    }
}

impl Song {
    pub fn artists_key(id: &str) -> String {
        format!("{}:artists", Song::key(id))
    }

    pub fn album(&self, db: &redis::Connection) -> FieldResult<Album> {
        Album::from_id(&self.album_id, db)
    }

    pub fn artists(&self, db: &redis::Connection) -> FieldResult<Vec<Artist>> {
        read_vec_from_db(&Song::artists_key(&self.id), db)
    }

    pub fn stats(&self, db: &redis::Connection) -> FieldResult<SongUserStats> {
        SongUserStats::from_id(&self.stat_id, db)
    }
}

impl Playlist {
    pub fn items_key(id: &str) -> String {
        format!("{}:items", Playlist::key(id))
    }

    pub fn items(
        &self,
        query: &ConnectionQuery,
        db: &redis::Connection,
    ) -> FieldResult<Connection<PlaylistItem>> {
        let limit = if query.cursor.is_none() {
            query.limit as usize
        } else {
            (query.limit + 1) as usize
        };

        let mut items: Vec<Edge<PlaylistItem>> = redis::cmd("LRANGE")
            .arg(Playlist::items_key(&self.id))
            .arg(0)
            .arg(-1)
            .iter::<String>(db)?
            .map(|item| {
                let node = PlaylistItem::from_id(&item, db).unwrap();
                Edge { cursor: item, node }
            })
            .skip_while(|edge| {
                query
                    .cursor
                    .as_ref()
                    .map(|item| *item != edge.cursor)
                    .unwrap_or(false)
            })
            .take(limit)
            .collect();

        // If we start with a cursor, ignore the item the cursor points to
        if !query.cursor.is_none() {
            items.remove(0);
        }

        Ok(Connection {
            count: items.len(),
            edges: items,
        })
    }

    pub fn duration(&self, db: &redis::Connection) -> FieldResult<i32> {
        Ok(redis::cmd("LRANGE")
            .arg(Playlist::items_key(&self.id))
            .arg(0)
            .arg(-1)
            .iter::<String>(db)?
            .map(|item| {
                let node = PlaylistItem::from_id(&item, db).unwrap();
                db.hget::<_, _, i32>(Song::key(&node.song_id), "duration")
                    .unwrap()
            })
            .sum())
    }
}

impl PlaylistItem {
    pub fn song(&self, db: &redis::Connection) -> FieldResult<Song> {
        Song::from_id(&self.song_id, db)
    }
}
