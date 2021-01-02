use crate::context::GraphQLContext;
use crate::database::album;
use crate::database::artist;
use crate::database::song;
use crate::models::*;
use chrono::Utc;
use diesel::expression::dsl::not;
use diesel::prelude::*;
use diesel::result;
use diesel::Connection;
use juniper::{FieldError, FieldResult};

pub mod errors {
    error_chain! {
        foreign_links {
            Diesel(::diesel::result::Error);
        }

        errors {
            MultipleDescriptors {
                description("multiple valid descriptors were passed. Only one should be passed")
            }
        }
    }
}

pub struct Mutation;

#[graphql_object(context = GraphQLContext)]
impl Mutation {
    fn play_song(
        &self,
        context: &GraphQLContext,
        song_id: UUID,
        artist_id: Option<UUID>,
        album_id: Option<UUID>,
    ) -> FieldResult<StatsCollection> {
        let conn = &context.connection() as &SqliteConnection;

        let valid_descriptors = vec![&artist_id, &album_id]
            .into_iter()
            .filter(|option| option.is_some())
            .count();

        if valid_descriptors > 1 {
            return Err(errors::ErrorKind::MultipleDescriptors.into());
        }

        let now = Utc::now().naive_utc();

        conn.transaction::<_, result::Error, _>(|| {
            if let Some(ref artist_id) = artist_id {
                diesel::update(artist::table.filter(artist::id.eq(artist_id)))
                    .set(artist::last_played.eq(now))
                    .execute(conn)?;
            }

            if let Some(ref album_id) = album_id {
                diesel::update(album::table.filter(album::id.eq(album_id)))
                    .set(album::last_played.eq(now))
                    .execute(conn)?;
            }

            diesel::update(song::table.filter(song::id.eq(song_id)))
                .set((
                    song::play_count.eq(song::play_count + 1),
                    song::last_played.eq(now),
                ))
                .execute(conn)?;

            Ok(())
        })?;

        Ok(StatsCollection {
            song_id,
            artist_id,
            album_id,
        })
    }

    fn toggle_like(&self, context: &GraphQLContext, song_id: UUID) -> FieldResult<Song> {
        {
            let conn = &context.connection() as &SqliteConnection;

            diesel::update(song::table.filter(song::id.eq(song_id)))
                .set(song::liked.eq(not(song::liked)))
                .execute(conn)?;

            // Drop the connection mutex guard before loading the song
        }

        Song::from_id(context, song_id).map_err(FieldError::from)
    }
}
