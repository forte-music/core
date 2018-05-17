use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;

use database::song;
use diesel;
use diesel::expression::dsl::not;
use diesel::prelude::*;

pub struct Mutation;

impl Mutation {
    pub fn play_song(context: &GraphQLContext, song_id: ID, artist_id: Option<ID>, album_id: Option<ID>, playlist_id: Option<ID>) -> FieldResult<StatsCollection> {
        NotImplementedErr()
    }

    pub fn toggle_like(context: &GraphQLContext, song_id: &str) -> FieldResult<Song> {
        let conn = &*context.connection;

        diesel::update(song::table.filter(song::id.eq(song_id)))
            .set(song::liked.eq(not(song::liked)))
            .execute(conn)?;

        Ok(Song::from_id(context, song_id)?)
    }
}

graphql_object!(
    Mutation: GraphQLContext | &self | {

    field play_song(&executor, song_id: ID, artist_id: Option<ID>, album_id: Option<ID>, playlist_id: Option<ID>) -> FieldResult<StatsCollection> {
        Mutation::play_song(executor.context(), song_id, artist_id, album_id, playlist_id)
    }

    field toggle_like(&executor, song_id: ID) -> FieldResult<Song> {
        Mutation::toggle_like(executor.context(), &song_id)
    }

    }
);
