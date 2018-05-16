use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;

use database::song;
use diesel;
use diesel::expression::dsl::not;
use diesel::prelude::*;

pub struct Mutation;

impl Mutation {
    pub fn play_song(context: &GraphQLContext, song_id: &str) -> FieldResult<SongUserStats> {
        let conn = &*context.connection;

        diesel::update(song::table.filter(song::id.eq(song_id)))
            .set(song::play_count.eq(song::play_count + 1))
            .execute(conn)?;

        let song = Song::from_id(context, song_id)?;
        Ok(song.stats(context))
    }

    pub fn toggle_like(context: &GraphQLContext, song_id: &str) -> FieldResult<SongUserStats> {
        let conn = &*context.connection;

        diesel::update(song::table.filter(song::id.eq(song_id)))
            .set(song::liked.eq(not(song::liked)))
            .execute(conn)?;

        let song = Song::from_id(context, song_id)?;
        Ok(song.stats(context))
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

    pub fn add_to_playlist_relative(
        context: &GraphQLContext,
        input: PlaylistAppendInput,
        position: Position,
    ) -> FieldResult<Playlist> {
        NotImplementedErr()
    }

    pub fn add_to_playlist_by_cursor(
        context: &GraphQLContext,
        input: PlaylistAppendInput,
        relative_to: ID,
        offset: Offset,
    ) -> FieldResult<Playlist> {
        NotImplementedErr()
    }

    pub fn add_to_playlist_by_index(
        context: &GraphQLContext,
        input: PlaylistAppendInput,
        position: i32,
        offset: Offset,
    ) -> FieldResult<Playlist> {
        NotImplementedErr()
    }

    pub fn remove_from_playlist(
        context: &GraphQLContext,
        playlist_id: ID,
        items: Vec<ID>,
    ) -> FieldResult<Playlist> {
        NotImplementedErr()
    }

    pub fn delete_playlist(context: &GraphQLContext, playlist_id: ID) -> FieldResult<bool> {
        NotImplementedErr()
    }

    pub fn move_song_in_playlist(
        context: &GraphQLContext,
        playlist_id: ID,
        from_item: ID,
        relative_to_item: ID,
        offset: Offset,
    ) -> FieldResult<Playlist> {
        NotImplementedErr()
    }
}

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

    field add_to_playlist_relative(
        &executor,
        input: PlaylistAppendInput,
        position: Position
    ) -> FieldResult<Playlist> {
        Mutation::add_to_playlist_relative(executor.context(), input, position)
    }

    field add_to_playlist_by_cursor(
        &executor,
        input: PlaylistAppendInput,
        relative_to: ID,
        offset: Offset
    ) -> FieldResult<Playlist> {
        Mutation::add_to_playlist_by_cursor(executor.context(), input, relative_to, offset)
    }

    field add_to_playlist_by_index(
        &executor,
        input: PlaylistAppendInput,
        position: i32,
        offset: Offset
    ) -> FieldResult<Playlist> {
        Mutation::add_to_playlist_by_index(executor.context(), input, position, offset)
    }

    field remove_from_playlist(
        &executor,
        playlist_id: ID,
        items: Vec<ID>
    ) -> FieldResult<Playlist> {
        Mutation::remove_from_playlist(executor.context(), playlist_id, items)
    }

    field delete_playlist(&executor, playlist_id: ID) -> FieldResult<bool> {
        Mutation::delete_playlist(executor.context(), playlist_id)
    }

    field move_song_in_playlist(
        &executor,
        playlist_id: ID,
        from_item: ID,
        relative_to_item: ID,
        offset: Offset
    ) -> FieldResult<Playlist> {
        Mutation::move_song_in_playlist(
            executor.context(),
            playlist_id,
            from_item,
            relative_to_item,
            offset
        )
    }

    }
);
