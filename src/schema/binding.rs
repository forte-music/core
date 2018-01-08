use juniper::{ID, FieldResult};
use schema::model::*;
use database;

#[derive(GraphQLInputObject)]
pub struct ConnectionQuery {
    #[graphql(
        description = "The maximum number of results to return.",
        default = "25"
    )]
    pub limit: i32,

    #[graphql(
        description = "The order in which the results are sorted in. By default they are sorted \
                       from most recently added to least recently added, unless otherwise \
                       specified."
    )]
    pub sort_by: Option<SortBy>,

    #[graphql(
        description = "Returns the results sorted in reverse order.",
        default = "false"
    )]
    pub reverse: bool,

    #[graphql(
        description = "Results after this cursor (Edge.cursor) will be returned. If not specified, \
                       starts from the first position.",
        default = "String::new()"
    )]
    pub cursor: String,

    #[graphql(
        description = "Only results with titles matching this string are returned. If this is \
                       specified, the default sortBy will now be from best match to worst match.",
        default = "String::new()"
    )]
    pub filter: String
}

#[derive(GraphQLInputObject)]
pub struct PlaylistAppendInput {
    #[graphql(
        description = "The id of the playlist to add songs to."
    )]
    pub playlist_id: ID,

    #[graphql(
        description = "The ids (Song.id) of songs to add to the playlist in the order specified."
    )]
    pub songs: Vec<ID>
}

// todo: add parameter documentation
graphql_object!(Query: database::Connection |&self| {
    field album(&executor, id: ID) -> FieldResult<Album>
            as "Get an album by its globally unique id." {
        Query::album(executor.context(), id)
    }

    field albums(&executor, input: ConnectionQuery) -> FieldResult<Connection<Album>>
            as "Get paginated, filtered, sorted albums." {
        Query::albums(executor.context())
    }

    field artist(&executor, id: ID) -> FieldResult<Artist>
            as "Get an artist by its globally unique id." {
        Query::artist(executor.context(), id)
    }

    field artists(&executor, input: ConnectionQuery) -> FieldResult<Connection<Artist>>
            as "Get paginated, filtered, sorted artists." {
        Query::artists(executor.context())
    }

    field song(&executor, id: ID) -> FieldResult<Song>
            as "Get a song by its globally unique id." {
        Query::song(executor.context(), id)
    }

    field songs(&executor, input: ConnectionQuery) -> FieldResult<Connection<Song>>
            as "Get paginated, filtered, sorted songs." {
        Query::songs(executor.context())
    }

    field playlist(&executor, id: ID) -> FieldResult<Playlist>
            as "Get a playlist by its globally unique id." {
        Query::playlist(executor.context(), id)
    }

    field playlists(&executor, input: ConnectionQuery) -> FieldResult<Connection<Playlist>>
            as "Get paginated, filtered, sorted playlists." {
        Query::playlists(executor.context())
    }
});

graphql_object!(Mutation: database::Connection |&self| {
    field play_song(&executor, song_id: ID) -> FieldResult<bool>
            as "Increments the play count and updates the last played time in SongUserStats.
                Always returns true." {
        Mutation::play_song(executor.context(), song_id)
    }

    field toggle_like(&executor, song_id: ID) -> FieldResult<bool>
            as "Toggles the like state of the specified song. Returns whether or not the song is \
                liked after the like is toggled." {
        Mutation::toggle_like(executor.context(), song_id)
    }

    field create_playlist(
        name: String
            as "The name of the new playlist.",
        songs: Option<Vec<ID>>
            as "The ids of songs to add to the playlist in the order specified. If an invalid id \
                is passed, the entire request fails."
    ) -> Playlist
            as "Creates a new playlist. Returns the newly created playlist." {
        Playlist::default()
    }

    field update_playlist(id: ID, name: String) -> Playlist
            as "Renames a playlist. Returns a playlist with the changes applied." {
        Playlist::default()
    }

    field add_to_playlist_relative(input: PlaylistAppendInput, position: Position) -> Playlist
            as "Adds songs to the end or the beginning of a playlist." {
        Playlist::default()
    }

    field add_to_playlist_by_cursor(
        input: PlaylistAppendInput,
        cursor: String
            as "The cursor relative to which to add songs (Playlist.songs.edges.cursor).",
        offset: Offset
            as "The direction relative to the cursor where songs will be added."
    ) -> Playlist
            as "Adds songs to a playlist relative to a cursor (Playlist.songs.edges.cursor). \
                This is useful because in some cases the index isn't known and is hard to compute \
                because of its dependence on global state while cursors are local state." {
        Playlist::default()
    }

    field add_to_playlist_by_index(
        input: PlaylistAppendInput,
        position: i32
            as "The zero indexed offset relative to which to add songs.",
        offset: Offset
            as "The direction relative to the offset where songs will be added."
    ) -> Playlist
            as "Adds songs to a playlist relative to an index." {
        Playlist::default()
    }

    field remove_from_playlist(
        playlist_id: ID
            as "The playlist to remove items from.",
        cursors: Vec<String>
            as "A list of cursors from Playlist.songs.edges.cursor pointing to songs to \
                remove from the playlist."
    ) -> Playlist
            as "Remove songs from the playlist. Returns the updated playlist." {
        Playlist::default()
    }

    field delete_playlist(id: ID) -> bool
            as "Permanently deletes a playlist." {
        true
    }

    field move_song_in_playlist(
        playlist_id: ID
            as "The id of the playlist to modify.",
        from_song_cursor: String
            as "The cursor (Playlist.songs.edges.cursor) of the element to move.",
        to_song_cursor: String
            as "The position the song will be moved relative to.",
        offset: Offset
            as "The direction relative to toSongCursor the song will be moved to."
    ) -> Playlist
            as "Moves a song in a playlist from one position to another." {
        Playlist::default()
    }
});

graphql_object!(Album: database::Connection |&self| {
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

    field artist(&executor) -> FieldResult<Artist>
            as "The artist who released the album. If there are multiple artists on the \
                album this is usually various artists (a designated id). This is usually the \
                album artists tag of files." {
        self.artist(executor.context())
    }

    field songs(&executor) -> FieldResult<Vec<Song>>
            as "Songs in this album sorted by song index." {
        self.songs(executor.context())
    }

    field duration(&executor) -> FieldResult<i32>
            as "The sum of the durations of every song in this album in seconds." {
        self.duration(executor.context())
    }

    field release_year() -> i32
            as "The year the album was released." {
        self.release_year
    }
});

graphql_object!(Artist: database::Connection |&self| {
    description: "An artist has a name and albums."

    field id() -> ID
            as "A globally unique id referring to this artist." {
        ID::from(self.id.clone())
    }

    field name() -> &str
            as "The human readable name of this artist." {
        &self.name
    }

    field albums(&executor) -> FieldResult<Vec<Album>>
            as "Albums this artist has authored. These are the albums that this artist is the \
                album artist of. The albums are sorted by release date." {
        self.albums(executor.context())
    }
});

graphql_object!(Song: database::Connection |&self| {
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

    field album(&executor) -> FieldResult<Album>
            as "The album this song is a part of. A song can only belong to one album." {
        self.album(executor.context())
    }

    field artists(&executor) -> FieldResult<Vec<Artist>>
            as "The artists which composed this song." {
        self.artists(executor.context())
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

    field stats(&executor) -> FieldResult<SongUserStats>
            as "User stats for a song." {
        self.stats(executor.context())
    }

    field duration() -> i32
            as "The duration of the song (retrievable at streamUrl) in seconds." {
        self.duration
    }
});

graphql_object!(SongUserStats: database::Connection |&self| {
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
                auto-playlist." {
        self.liked
    }
});

graphql_object!(Playlist: database::Connection |&self| {
    description: "A named collection of songs. The same song can appear multiple times in a \
                  playlist."

    field id() -> ID
            as "A globally unique id referring to this playlist." {
        ID::from(self.id.clone())
    }

    field name() -> &str
            as "Human readable name of the playlist. This is chosen by the user when the playlist \
                is created." {
        &self.name
    }

    field duration(&executor) -> FieldResult<i32>
            as "The sum of durations of every song in the playlist in seconds." {
        self.duration(executor.context())
    }

    field items(&executor, input: ConnectionQuery) -> FieldResult<Connection<PlaylistItem>>
            as "The items in the playlist." {
        self.items(input, executor.context())
    }
});

graphql_object!(PlaylistItem: database::Connection |&self| {
    description: "An item in a playlist."

    field id() -> ID
            as "The id of the playlist item. This is position invariant and allows for \
                addressing items in a playlist." {
        ID::from(self.id.clone())
    }

    field song(&executor) -> FieldResult<Song>
            as "The song this item points to." {
        self.song(executor.context())
    }
});

graphql_object!(Edge<Album>: database::Connection as "AlbumEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Album { &self.node }
});

graphql_object!(Edge<Artist>: database::Connection as "ArtistEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Artist { &self.node }
});

graphql_object!(Edge<Song>: database::Connection as "SongEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Song { &self.node }
});

graphql_object!(Edge<Playlist>: database::Connection as "PlaylistEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Playlist { &self.node }
});

graphql_object!(Edge<PlaylistItem>: database::Connection as "PlaylistItemEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &PlaylistItem { &self.node }
});

graphql_object!(Connection<Album>: database::Connection as "AlbumConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Album>] { &self.edges }
});

graphql_object!(Connection<Artist>: database::Connection as "ArtistConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Artist>] { &self.edges }
});

graphql_object!(Connection<Song>: database::Connection as "SongConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Song>] { &self.edges }
});

graphql_object!(Connection<Playlist>: database::Connection as "PlaylistConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Playlist>] { &self.edges }
});

graphql_object!(Connection<PlaylistItem>: database::Connection as "PlaylistItemConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<PlaylistItem>] { &self.edges }
});
