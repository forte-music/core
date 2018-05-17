extern crate chrono;

use chrono::Utc;

use context::GraphQLContext;
use juniper::{FieldError, FieldResult, ID};
use models::*;

use database::album;
use database::artist;
use database::playlist;
use database::song;

use diesel;
use diesel::Connection;
use diesel::expression::dsl::not;
use diesel::prelude::*;

pub struct Mutation;

impl Mutation {
    pub fn play_song(
        context: &GraphQLContext,
        song_id: ID,
        artist_id: Option<ID>,
        album_id: Option<ID>,
        playlist_id: Option<ID>,
    ) -> FieldResult<StatsCollection> {
        let conn = &*context.connection;

        let valid_descriptors = (vec![&artist_id, &album_id, &playlist_id])
            .iter()
            .filter(|option| option.is_some())
            .collect::<Vec<&&Option<ID>>>()
            .len();

        if valid_descriptors > 1 {
            return Err(FieldError::from(
                "Multiple valid descriptors were passed. Only one should be passed.",
            ));
        }

        let now = Utc::now().timestamp() as i32;

        conn.transaction::<_, FieldError, _>(|| {
            if let Some(ref artist_id) = artist_id {
                let artist_id: &str = artist_id.as_ref();
                diesel::update(artist::table.filter(artist::id.eq(artist_id)))
                    .set(artist::last_played.eq(now))
                    .execute(conn)?;
            }

            if let Some(ref album_id) = album_id {
                let album_id: &str = album_id.as_ref();
                diesel::update(album::table.filter(album::id.eq(album_id)))
                    .set(album::last_played.eq(now))
                    .execute(conn)?;
            }

            if let Some(ref playlist_id) = playlist_id {
                let playlist_id: &str = playlist_id.as_ref();
                diesel::update(playlist::table.filter(playlist::id.eq(playlist_id)))
                    .set(playlist::last_played.eq(now))
                    .execute(conn)?;
            }

            let song_id: &str = song_id.as_ref();
            diesel::update(song::table.filter(song::id.eq(song_id)))
                .set(song::play_count.eq(song::play_count + 1))
                .execute(conn)?;

            Ok(())
        })?;

        Ok(StatsCollection {
            song_id,
            artist_id,
            album_id,
            playlist_id,
        })
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

    field play_song(
        &executor,
        song_id: ID,
        artist_id: Option<ID>,
        album_id: Option<ID>,
        playlist_id: Option<ID>
    ) -> FieldResult<StatsCollection> {
        Mutation::play_song(executor.context(), song_id, artist_id, album_id, playlist_id)
    }

    field toggle_like(&executor, song_id: ID) -> FieldResult<Song> {
        Mutation::toggle_like(executor.context(), &song_id)
    }
});
