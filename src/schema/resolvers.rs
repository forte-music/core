use std::ops::Deref;
use schema::model::*;
use juniper::{ID, FieldResult, FieldError, Value};
use redis::{self, Commands};

impl Query {
    pub fn album(db: &redis::Connection, id: ID) -> FieldResult<Album> {
        let key = format!("album:{}", id.deref());

        if !db.exists::<&str, bool>(&key)? {
            return Err(FieldError::new("Album does not exist", Value::String(id.to_string())));
        }

        Ok(Album::from_id(id.to_string()))
    }

    pub fn artist(id: ID) -> Artist {
        Artist {
            id: id.to_string(),
            name: "artist_name".to_owned(),
            albums: vec![],
            featured: vec![],
            singles: vec![]
        }
    }

    pub fn song(id: ID) -> Song {
        Song {
            id: id.to_string(),
            name: "song_name".to_owned(),
            album: Album {
                id: "album_id".to_owned(),
                artwork_url: None,
                name: "album_name".to_owned(),
                artist: Artist {
                    id: "artist_id".to_owned(),
                    name: "artist_name".to_owned(),
                    albums: vec![],
                    featured: vec![],
                    singles: vec![]
                },
                songs: vec![]
            },
            artists: vec![],
            stream_url: "stream_url".to_owned(),
            track_number: 0,
            disk_number: 0,
            stats: SongUserStats {
                id: "0".to_string(),
                play_count: 0,
                last_played: 0,
                liked: false
            }
        }
    }

    pub fn generic_connection<T>(limit: i32, cursor: Option<String>) -> Connection<T> {
        Connection {
            count: 0,
            edges: vec![]
        }
    }
}
