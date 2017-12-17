pub trait FromId {
    fn from_id(id: String) -> Self;
}

pub struct Query;
pub struct Mutation;

pub struct Album {
    pub id: String,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist: Artist,
    pub songs: Vec<Song>
}

impl FromId for Album {
    fn from_id(id: String) -> Album {
        Album {
            id, .. Album::default()
        }
    }
}

impl Default for Album {
    fn default() -> Self {
        Album {
            id: "0".to_owned(),
            artwork_url: None,
            name: "".to_owned(),
            artist: Artist::default(),
            songs: vec![]
        }
    }
}

pub struct Artist {
    pub id: String,
    pub name: String,
    pub albums: Vec<Album>,
    pub featured: Vec<Album>,
    pub singles: Vec<Album>
}

impl FromId for Artist {
    fn from_id(id: String) -> Self {
        Artist { id, .. Artist::default() }
    }
}

impl Default for Artist {
    fn default() -> Self {
        Artist {
            id: "0".to_owned(),
            name: "".to_owned(),
            albums: vec![],
            featured: vec![],
            singles: vec![]
        }
    }
}

pub struct Song {
    pub id: String,
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub stream_url: String,
    pub track_number: i32,
    pub disk_number: i32,
    pub stats: SongUserStats
}

impl FromId for Song {
    fn from_id(id: String) -> Self {
        Song { id, .. Song::default() }
    }
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: "0".to_owned(),
            name: "".to_owned(),
            album: Album::default(),
            artists: vec![],
            stream_url: "".to_owned(),
            track_number: 0,
            disk_number: 0,
            stats: SongUserStats::default()
        }
    }
}

pub struct SongUserStats {
    pub id: String,
    pub play_count: i32,
    pub last_played: i32,
    pub liked: bool
}

impl FromId for SongUserStats {
    fn from_id(id: String) -> Self {
        SongUserStats { id, .. SongUserStats::default() }
    }
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

impl FromId for Playlist {
    fn from_id(id: String) -> Self {
        Playlist { id, .. Playlist::default() }
    }
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
    MostPlayed
}
