use schema::model::*;
use juniper;
use db;

pub struct Query;

// todo: add parameter documentation, remove stubs and use database
graphql_object!(Query: db::Connection |&self| {
    field album(id: juniper::ID) -> Album as "Get an album by its globally unique id." {
        album(id)
    }

    field artist(id: juniper::ID) -> Artist as "Get an artist by its globally unique id." {
        artist(id)
    }

    field song(id: juniper::ID) -> Song as "Get a song by its globally unique id." {
        song(id)
    }

    field albums(limit = 25: i32, cursor: Option<String>, sort_by = (SortBy::RecentlyAdded): SortBy)
            -> Connection<Album> as "Get paginated, sorted albums." {
        generic_connection(limit, cursor)
    }

    field search_albums(name: String, limit = 25: i32, cursor: Option<String>) -> Connection<Album>
            as "Search only for albums by name." {
        generic_connection(limit, cursor)
    }
});

fn album(id: juniper::ID) -> Album {
    Album {
        id, artwork_url: None, name: "album_name".to_string(),
        artist: Artist {
            id: juniper::ID::from("artist_id".to_string()), name: "artist_name".to_string(),
            albums: vec![], featured: vec![], singles: vec![]
        },
        songs: vec![]
    }
}

fn artist(id: juniper::ID) -> Artist {
    Artist {
        id, name: "artist_name".to_string(),
        albums: vec![], featured: vec![], singles: vec![]
    }
}

fn song(id: juniper::ID) -> Song {
    Song {
        id, name: "song_name".to_string(),
        album: Album {
            id: juniper::ID::from("album_id".to_string()), artwork_url: None, name: "album_name".to_string(),
            artist: Artist {
                id: juniper::ID::from("artist_id".to_string()), name: "artist_name".to_string(),
                albums: vec![], featured: vec![], singles: vec![]
            },
            songs: vec![]
        },
        artists: vec![],
        stream_url: "stream_url".to_string(), track_number: 0, disk_number: 0,
        stats: SongUserStats {
            id: juniper::ID::from("0".to_string()),
            play_count: 0, last_played: 0, liked: false
        }
    }
}

fn generic_connection<T>(limit: i32, cursor: Option<String>) -> Connection<T> {
    Connection {
        count: 0,
        edges: vec![]
    }
}

pub struct Mutation;

graphql_object!(Mutation: db::Connection |&self| {
    // todo: add documentation, remove stubs, and use database

    field like(id: juniper::ID) -> bool {
        true
    }

});
