use juniper::{ID, FieldResult};
use schema::model::*;
use database;

// todo: add parameter documentation
graphql_object!(Query: database::Connection |&self| {
    field album(&executor, id: ID) -> FieldResult<Album>
            as "Get an album by its globally unique id." {
        Query::album(&executor.context(), id)
    }

    field artist(id: ID) -> Artist
            as "Get an artist by its globally unique id." {
        Query::artist(id)
    }

    field song(id: ID) -> Song
            as "Get a song by its globally unique id." {
        Query::song(id)
    }

    field albums(limit = 25: i32, cursor: Option<String>, sort_by = (SortBy::RecentlyAdded): SortBy)
            -> Connection<Album>
            as "Get paginated, sorted albums." {
        Query::generic_connection(limit, cursor)
    }

    field search_albums(name: String, limit = 25: i32, cursor: Option<String>) -> Connection<Album>
            as "Search only for albums by name." {
        Query::generic_connection(limit, cursor)
    }
});

graphql_object!(Mutation: database::Connection |&self| {
    // todo: add documentation, remove stubs, and use database

    field like(id: ID) -> bool {
        true
    }

});

graphql_object!(Album: () |&self| {
    description: "An album is a collection of songs which belong to an artist and has a name."

    field id() -> ID
            as "A globally unique id referring to this album." {
        ID::from(self.id.clone())
    }

    field artwork_url() -> &Option<String>
            as "The http/https url at which a square PNG of the album artwork can be found. \
                Clients should request artwork with the same authentication as used with the \
                API server." {
        &self.artwork_url
    }

    field name() -> &str
            as "The human readable name of the album." {
        &self.name
    }

    field artist() -> &Artist
            as "The artist who released the album. If there are multiple artists on the \
                album this is usually various artists (a designated id). This is usually the \
                album artists tag of files." {
        &self.artist
    }

    field songs() -> &[Song]
            as "Songs in this album sorted by song index." {
        &self.songs
    }
});

graphql_object!(Artist: () |&self| {
    description: "An artist has a name and albums."

    field id() -> ID
            as "A globally unique id referring to this artist." {
        ID::from(self.id.clone())
    }

    field name() -> &str
            as "The human readable name of this artist." {
        &self.name
    }

    field albums() -> &[Album]
            as "Albums this artist has authored. These are the albums that this artist is the \
                album artist of." {
        &self.albums
    }

    field featured() -> &[Album]
            as "The albums which this artist has featured on. These are albums which the artist \
                isn't an album artist of but albums which the artist is in." {
        &self.featured
    }

    field singles() -> &[Album]
            as "Albums with only a single song where this artist is the album artist or an artist \
                of the song." {
        &self.singles
    }
});

graphql_object!(Song: () |&self| {
    description: "A song is a piece of music written by artists. It is always part of an album. \
                  It represents a singe audio file."

    field id() -> ID
            as "A globally unique id referring to this song." {
        ID::from(self.id.clone())
    }

    field name() -> &str
            as "The human readable name of this song." {
        &self.name
    }

    field album() -> &Album
            as "The album this song is a part of. A song can only belong to one album." {
        &self.album
    }

    field artists() -> &[Artist]
            as "The artists which composed this song." {
        &self.artists
    }

    field stream_url() -> &str
            as "The url at which the song can be streamed from. See \
                github.com/forte-music/schema for details about this field." {
        &self.stream_url
    }

    field track_number() -> i32
            as "The track number of this song. This may be one or zero indexed and is provided by \
                the file's metadata." {
        self.track_number
    }

    field disk_number() -> i32
            as "The disk this track is on. The disk number is assumed to be one if not provided." {
        self.disk_number
    }

    field stats() -> &SongUserStats
            as "User stats for a song." {
        &self.stats
    }
});

graphql_object!(SongUserStats: () |&self| {
    description: "Stats for a song tied to a specific user."

    field id() -> ID
            as "A globally unique id referring to a song's stats." {
        ID::from(self.id.clone())
    }

    field play_count() -> i32
            as "The number of times this song has been played." {
        self.play_count
    }

    field last_played() -> i32
            as "The epoch time (seconds) at which this song was last played." {
        self.last_played
    }

    field liked() -> bool
            as "Whether or not this song is favorited. Favorited songs go into their own \
                playlist." {
        self.liked
    }
});

graphql_object!(Playlist: database::Connection |&self| {
    description: "A named collection of songs."

    field id() -> ID
            as "A globally unique id referring to this playlist." {
        ID::from(self.id.clone())
    }

    field name() -> &str
            as "Human readable name of the playlist. This is chosen by the user when the playlist \
                is created." {
        &self.name
    }

    field songs(limit: i32, cursor: String) -> Connection<Song>
            as "An ordered list of songs in the playlist." {
        Connection { count: 0, edges: vec![] }
    }
});

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
