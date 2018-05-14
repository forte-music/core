// The GraphQL macros like their lint warnings
#![allow(unused_parens)]
#![cfg_attr(feature = "cargo-clippy", allow(double_parens, op_ref))]

use juniper::{FieldResult, ID};
use schema::model::*;
use context;

#[derive(GraphQLEnum)]
pub enum SortBy {
    #[graphql(name = "RECENTLY_ADDED")] RecentlyAdded,
    #[graphql(name = "LEXICOGRAPHICALLY")] Lexicographically,
    #[graphql(name = "RELEVANCE")] Relevance,
}

#[derive(GraphQLEnum)]
pub enum Position {
    #[graphql(name = "BEGINNING")] Beginning,
    #[graphql(name = "END")] End,
}

#[derive(GraphQLEnum)]
pub enum Offset {
    #[graphql(name = "AFTER")] After,
    #[graphql(name = "BEFORE")] Before,
}

#[derive(GraphQLInputObject)]
pub struct ConnectionQuery {
    #[graphql(default = "25")]
    pub limit: i32,

    #[graphql(default = "None")]
    pub cursor: Option<String>,
}

#[derive(GraphQLInputObject)]
pub struct SortParams {
    pub sort_by: SortBy,

    #[graphql(default = "false")]
    pub reverse: bool,

    #[graphql(default = "String::new()")]
    pub filter: String,
}

#[derive(GraphQLInputObject)]
pub struct PlaylistAppendInput {
    pub playlist_id: ID,
    pub songs: Vec<ID>,
}

graphql_object!(Query: database::Connection |&self| {
    field album(&executor, id: ID) -> FieldResult<Album> {
        Query::album(executor.context(), &id)
    }

    field albums(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Album>> {
        Query::albums(executor.context())
    }

    field artist(&executor, id: ID) -> FieldResult<Artist> {
        Query::artist(executor.context(), &id)
    }

    field artists(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Artist>> {
        Query::artists(executor.context())
    }

    field song(&executor, id: ID) -> FieldResult<Song> {
        Query::song(executor.context(), &id)
    }

    field songs(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Song>> {
        Query::songs(executor.context())
    }

    field playlist(&executor, id: ID) -> FieldResult<Playlist> {
        Query::playlist(executor.context(), &id)
    }

    field playlists(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Playlist>> {
        Query::playlists(executor.context())
    }
});

graphql_object!(Mutation: database::Connection |&self| {
    field play_song(&executor, song_id: ID) -> FieldResult<SongUserStats> {
        Mutation::play_song(executor.context(), &song_id)
    }

    field toggle_like(&executor, song_id: ID) -> FieldResult<SongUserStats> {
        Mutation::toggle_like(executor.context(), &song_id)
    }

    field create_playlist(
        name: String,
        songs: Option<Vec<ID>>
    ) -> Playlist {
        Playlist::default()
    }

    field update_playlist(playlist_id: ID, name: String) -> Playlist {
        Playlist::default()
    }

    field add_to_playlist_relative(input: PlaylistAppendInput, position: Position) -> Playlist {
        Playlist::default()
    }

    field add_to_playlist_by_cursor(
        input: PlaylistAppendInput,
        relative_to: ID,
        offset: Offset
    ) -> Playlist {
        Playlist::default()
    }

    field add_to_playlist_by_index(
        input: PlaylistAppendInput,
        position: i32,
        offset: Offset
    ) -> Playlist {
        Playlist::default()
    }

    field remove_from_playlist(
        playlist_id: ID,
        items: Vec<ID>
    ) -> Playlist {
        Playlist::default()
    }

    field delete_playlist(playlist_id: ID) -> bool {
        true
    }

    field move_song_in_playlist(
        playlist_id: ID,
        from_item: ID,
        relative_to_item: ID,
        offset: Offset
    ) -> Playlist {
        Playlist::default()
    }
});

graphql_object!(Album: database::Connection |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field artwork_url() -> &Option<String> {
        &self.artwork_url
    }

    field name() -> &str {
        &self.name
    }

    field artist(&executor) -> FieldResult<Artist> {
        self.artist(executor.context())
    }

    field songs(&executor) -> FieldResult<Vec<Song>> {
        self.songs(executor.context())
    }

    field duration(&executor) -> FieldResult<i32> {
        self.duration(executor.context())
    }

    field release_year() -> i32 {
        self.release_year
    }

    field time_added() -> i32 {
        self.time_added
    }
});

graphql_object!(Artist: database::Connection |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field name() -> &str {
        &self.name
    }

    field albums(&executor) -> FieldResult<Vec<Album>> {
        self.albums(executor.context())
    }

    field time_added() -> i32 {
        self.time_added
    }
});

graphql_object!(Song: database::Connection |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field name() -> &str {
        &self.name
    }

    field album(&executor) -> FieldResult<Album> {
        self.album(executor.context())
    }

    field artists(&executor) -> FieldResult<Vec<Artist>> {
        self.artists(executor.context())
    }

    field stream_url() -> &str {
        &self.stream_url
    }

    field track_number() -> i32 {
        self.track_number
    }

    field disk_number() -> i32 {
        self.disk_number
    }

    field stats(&executor) -> FieldResult<SongUserStats> {
        self.stats(executor.context())
    }

    field duration() -> i32 {
        self.duration
    }

    field time_added() -> i32 {
        self.time_added
    }
});

graphql_object!(SongUserStats: database::Connection |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field play_count() -> i32 {
        self.play_count
    }

    field last_played() -> Option<i32> {
        self.last_played
    }

    field liked() -> bool {
        self.liked
    }
});

graphql_object!(Playlist: database::Connection |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field name() -> &str {
        &self.name
    }

    field duration(&executor) -> FieldResult<i32> {
        self.duration(executor.context())
    }

    field time_added() -> i32 {
        self.time_added
    }

    field items(&executor, input: ConnectionQuery) -> FieldResult<Connection<PlaylistItem>> {
        self.items(&input, executor.context())
    }
});

graphql_object!(PlaylistItem: database::Connection |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field song(&executor) -> FieldResult<Song> {
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
