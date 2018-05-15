use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;
use std::io::Error;

pub struct Mutation;

impl Mutation {
    /*
    pub fn play_song(context: GraphQLContext, song_id: &str) -> Result<SongUserStats, Box<Error>> {
    }

    pub fn toggle_like(context: GraphQLContext, song_id: &str) -> Result<SongUserStats, Box<Error>> {
    }
    */
}

// TODO: Implement Missing Mutations
graphql_object!(
    Mutation: GraphQLContext | &self | {
        /*
    field play_song(&executor, song_id: ID) -> FieldResult<SongUserStats> {
        Mutation::play_song(executor.context(), &song_id)
    }

    field toggle_like(&executor, song_id: ID) -> FieldResult<SongUserStats> {
        Mutation::toggle_like(executor.context(), &song_id)
    }
    */

        /*
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
    */
    }
);
