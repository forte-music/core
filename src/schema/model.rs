pub struct Query;
pub struct Mutation;

#[derive(Deserialize)]
pub struct Album {
    pub id: String,
    pub artwork_url: Option<String>,
    pub name: String,
//    pub artist: Artist,
//    pub songs: Vec<Song>
}

impl Default for Album {
    fn default() -> Self {
        Album {
            id: "0".to_owned(),
            artwork_url: None,
            name: "".to_owned(),
//            artist: Artist::default(),
//            songs: vec![]
        }
    }
}

#[derive(Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
//    pub albums: Vec<Album>,
//    pub featured: Vec<Album>,
//    pub singles: Vec<Album>
}

impl Default for Artist {
    fn default() -> Self {
        Artist {
            id: "0".to_owned(),
            name: "".to_owned(),
//            albums: vec![],
//            featured: vec![],
//            singles: vec![]
        }
    }
}

#[derive(Deserialize)]
pub struct Song {
    pub id: String,
    pub name: String,
//    pub album: Album,
//    pub artists: Vec<Artist>,
    pub stream_url: String,
    pub track_number: i32,
    pub disk_number: i32,
//    pub stats: SongUserStats
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: "0".to_owned(),
            name: "".to_owned(),
//            album: Album::default(),
//            artists: vec![],
            stream_url: "".to_owned(),
            track_number: 0,
            disk_number: 0,
//            stats: SongUserStats::default()
        }
    }
}

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
