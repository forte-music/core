pub struct Query;
pub struct Mutation;

#[derive(Deserialize)]
pub struct Album {
    pub id: String,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist_id: String,
    pub release_year: i32
}

impl Default for Album {
    fn default() -> Self {
        Album {
            id: "0".to_owned(),
            artwork_url: None,
            name: "".to_owned(),
            artist_id: "".to_owned(),
            release_year: 0
        }
    }
}

#[derive(Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

impl Default for Artist {
    fn default() -> Self {
        Artist {
            id: "0".to_owned(),
            name: "".to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub album_id: String,
    pub stat_id: String,
    pub stream_url: String,
    pub track_number: i32,
    pub disk_number: i32,
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: "0".to_owned(),
            name: "".to_owned(),
            album_id: "0".to_owned(),
            stat_id: "0".to_owned(),
            stream_url: "".to_owned(),
            track_number: 0,
            disk_number: 0,
        }
    }
}

#[derive(Deserialize)]
pub struct SongUserStats {
    pub id: String,
    pub play_count: i32,
    pub last_played: i32,
    pub liked: bool
}

impl Default for SongUserStats {
    fn default() -> Self {
        SongUserStats {
            id: "0".to_owned(),
            play_count: 0,
            last_played: 0,
            liked: false
        }
    }
}

pub struct Playlist {
    pub id: String,
    pub name: String,
}

impl Default for Playlist {
    fn default() -> Self {
        Playlist {
            id: "0".to_owned(),
            name: "".to_owned()
        }
    }
}

pub struct Edge<T> {
    pub cursor: String,
    pub node: T
}

pub struct Connection<T> {
    pub count: i32,
    pub edges: Vec<Edge<T>>
}

impl<T> Default for Connection<T> {
    fn default() -> Self {
        Connection {
            count: 0,
            edges: vec![]
        }
    }
}

#[derive(GraphQLEnum)]
pub enum SortBy {
    #[graphql(
        name = "RECENTLY_ADDED",
        description = "Sort from most recently added to least recently added."
    )]
    RecentlyAdded,

    #[graphql(
        name = "RECENTLY_PLAYED",
        description = "Sort from most recently played to least recently played."
    )]
    RecentlyPlayed,

    #[graphql(
        name = "MOST_PLAYED",
        description = "Sort from most played to least played."
    )]
    MostPlayed,

    #[graphql(
        description = "Sort by title in case-insensitive alphabetic order."
    )]
    Lexicographically
}

#[derive(GraphQLEnum)]
pub enum Position {
    #[graphql(
        description = "Elements are inserted before the beginning of the list."
    )]
    Beginning,

    #[graphql(
        description = "Elements are inserted after the end of the list."
    )]
    End
}

#[derive(GraphQLEnum)]
pub enum Offset {
    After,
    Before
}
