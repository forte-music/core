use juniper;

#[derive(GraphQLObject)]
#[graphql(
    description = "An album is a collection of songs which belong to an artist and has a name."
)]
pub struct Album {
    #[graphql(description = "A globally unique id referring to this album.")]
    pub id: juniper::ID,

    #[graphql(
        description =
            "The http/https url at which a square PNG of the album artwork can be found. \
             Clients should request artwork with the same authentication as used with the \
             API server."
    )]
    pub artwork_url: Option<String>,

    #[graphql(description = "The human readable name of the album.")]
    pub name: String,

    #[graphql(
        description =
            "The artist who released the album. If there are multiple artists on the \
             album this is usually various artists (a designated id). This is usually the \
             album artists tag of files."
    )]
    pub artist: Artist,

    #[graphql(description = "Songs in this album sorted by song index.")]
    pub songs: Vec<Song>
}

#[derive(GraphQLObject)]
#[graphql(description = "An artist has a name and albums.")]
pub struct Artist {
    #[graphql(description = "A globally unique id referring to this artist.")]
    pub id: juniper::ID,

    #[graphql(description = "A globally unique id referring to this artist.")]
    pub name: String,

    #[graphql(
        description =
            "Albums this artist has authored. These are the albums that this artist is \
             the album artist of."
    )]
    pub albums: Vec<Album>,

    #[graphql(
        description =
            "The albums which this artist has featured on. These are albums which the \
             artist isn't an album artist of but albums which the artist is in."
    )]
    pub featured: Vec<Album>,

    #[graphql(
        description =
            "Albums with only a single song where this artist is the album artist or an \
             artist of the song."
    )]
    pub singles: Vec<Album>
}

#[derive(GraphQLObject)]
#[graphql(
    description =
        "A song is a piece of music written by artists. It is always part of an album. \
         It represents a singe audio file."
)]
pub struct Song {
    #[graphql(description = "A globally unique id referring to this song.")]
    pub id: juniper::ID,

    #[graphql(description = "The human readable name of this song.")]
    pub name: String,

    #[graphql(
        description = "The album this song is a part of. A song can only belong to one album."
    )]
    pub album: Album,

    #[graphql(description = "The artists which composed this song.")]
    pub artists: Vec<Artist>,

    #[graphql(
        description =
            "The url at which the song can be streamed from. See \
             github.com/forte-music/schema for details about this field."
    )]
    pub stream_url: String,

    #[graphql(
        description =
            "The track number of this song. This may be one or zero indexed and is \
             provided by the file's metadata."
    )]
    pub track_number: i32,

    #[graphql(
        description =
            "The disk this track is on. The disk number is assumed to be one if not \
             provided."
    )]
    pub disk_number: i32,

    #[graphql(description = "User stats for a song.")]
    pub stats: SongUserStats
}

#[derive(GraphQLObject)]
#[graphql(description = "Stats for a song tied to a specific user.")]
pub struct SongUserStats {
    #[graphql(description = "A globally unique id referring to a song's stats.")]
    pub id: juniper::ID,

    #[graphql(description = "The number of times this song has been played.")]
    pub play_count: i32,

    #[graphql(description = "The epoch time (seconds) at which this song was last played.")]
    pub last_played: i32,

    #[graphql(
        description =
            "Whether or not this song is favorited. Favorited songs go into their own \
             playlist."
    )]
    pub liked: bool
}

pub struct Playlist {
    pub id: juniper::ID,
    pub name: String,
}

graphql_object!(Playlist: () |&self| {
    description: "A named collection of songs."

    field id() -> &juniper::ID as "A globally unique id referring to this playlist." {
        &self.id
    }

    field name() -> &str as "Human readable name of the playlist. This is chosen by the user when \
                             the playlist is created." {
        &self.name
    }

    field songs(limit: i32, cursor: String) -> Connection<Song>
            as "An ordered list of songs in the playlist." {
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
