use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;

pub struct Mutation;

impl Mutation {
    pub fn play_song(context: &GraphQLContext, song_id: &str) -> FieldResult<SongUserStats> {
        NotImplementedErr()
    }

    pub fn toggle_like(context: &GraphQLContext, song_id: &str) -> FieldResult<SongUserStats> {
        NotImplementedErr()
    }

    pub fn create_playlist(
        context: &GraphQLContext,
        songs: Option<Vec<ID>>,
    ) -> FieldResult<Playlist> {
        NotImplementedErr()
    }

    pub fn update_playlist(
        context: &GraphQLContext,
        playlist_id: ID,
        name: String,
    ) -> FieldResult<Playlist> {
        NotImplementedErr()
    }
}

// TODO: Implement Missing Mutations
graphql_object!(
    Mutation: GraphQLContext | &self | {
    field play_song(&executor, song_id: ID) -> FieldResult<SongUserStats> {
        Mutation::play_song(executor.context(), &song_id)
    }

    field toggle_like(&executor, song_id: ID) -> FieldResult<SongUserStats> {
        Mutation::toggle_like(executor.context(), &song_id)
    }

    field create_playlist(
        &executor,
        name: String,
        songs: Option<Vec<ID>>
    ) -> FieldResult<Playlist> {
        Mutation::create_playlist(executor.context(), songs)
    }

    field update_playlist(&executor, playlist_id: ID, name: String) -> FieldResult<Playlist> {
    Mutation::update_playlist(executor.context(), playlist_id, name)
    }

/*
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
    */
    }
);
