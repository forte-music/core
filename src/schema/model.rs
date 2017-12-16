#[derive(Deserialize)]
pub struct Album {
    pub id: String,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist: Artist,
    pub songs: Vec<Song>
}

#[derive(Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub albums: Vec<Album>,
    pub featured: Vec<Album>,
    pub singles: Vec<Album>
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct SongUserStats {
    pub id: String,
    pub play_count: i32,
    pub last_played: i32,
    pub liked: bool
}

// GraphQL binding located in resolvers
#[derive(Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Edge<T> {
    pub cursor: String,
    pub node: T
}

#[derive(Deserialize)]
pub struct Connection<T> {
    pub count: i32,
    pub edges: Vec<Edge<T>>
}

#[derive(GraphQLEnum, Deserialize)]
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
