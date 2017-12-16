use juniper;
use db;

#[derive(GraphQLObject)]
pub struct Album {
    pub id: juniper::ID,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist: Artist,
    pub songs: Vec<Song>
}

#[derive(GraphQLObject)]
pub struct Artist {
    pub id: juniper::ID,
    pub name: String,
    pub albums: Vec<Album>,
    pub featured: Vec<Album>,
    pub singles: Vec<Album>
}

#[derive(GraphQLObject)]
pub struct Song {
    pub id: juniper::ID,
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub stream_url: String,
    pub track_number: i32,
    pub disk_number: i32,
    pub stats: SongUserStats
}

#[derive(GraphQLObject)]
pub struct SongUserStats {
    pub id: juniper::ID,
    pub play_count: i32,
    pub last_played: i32,
    pub liked: bool
}

pub struct Playlist {
    pub id: juniper::ID,
    pub name: String,
}

graphql_object!(Playlist: () |&self| {
    field id() -> &juniper::ID { &self.id }

    field name() -> &str { &self.name }

    field songs(limit: i32, cursor: String) -> Connection<Song> {
        Connection { count: 0, edges: vec![] }
    }
});

pub struct Edge<T> {
    pub cursor: String,
    pub node: T
}

pub struct Connection<T> {
    pub count: i32,
    pub edges: Vec<Edge<T>>
}

graphql_object!(Edge<Album>: () as "AlbumEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Album { &self.node }
});

graphql_object!(Edge<Artist>: () as "ArtistEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Artist { &self.node }
});

graphql_object!(Edge<Song>: () as "SongEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Song { &self.node }
});

graphql_object!(Connection<Album>: () as "AlbumConnection" |&self| {
    field count() -> i32 { self.count }
    field edges() -> &[Edge<Album>] { &self.edges }
});

graphql_object!(Connection<Artist>: () as "AlbumConnection" |&self| {
    field count() -> i32 { self.count }
    field edges() -> &[Edge<Artist>] { &self.edges }
});

graphql_object!(Connection<Song>: () as "AlbumConnection" |&self| {
    field count() -> i32 { self.count }
    field edges() -> &[Edge<Song>] { &self.edges }
});

#[derive(GraphQLEnum)]
pub enum SortBy {
    #[graphql(name = "RECENTLY_ADDED")]
    RecentlyAdded,

    #[graphql(name = "RECENTLY_PLAYED")]
    RecentlyPlayed,

    #[graphql(name = "MOST_PLAYED")]
    MostPlayed
}

pub struct Query;

graphql_object!(Query: db::Connection |&self| {
    field album(id: juniper::ID) -> Album {
        // Just for testing
        Album {
            id, artwork_url: None, name: "album_name".to_string(),
            artist: Artist {
                id: juniper::ID::from("artist_id".to_string()), name: "artist_name".to_string(),
                albums: vec![], featured: vec![], singles: vec![]
            },
            songs: vec![]
        }
    }

    // ...
});

pub struct Mutation;

graphql_object!(Mutation: db::Connection |&self| {
    field like(id: juniper::ID) -> bool {
        true
    }

    // ...
});

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
