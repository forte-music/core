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
use juniper::FieldResult;

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

impl Mutation {
    pub fn play_song(
        context: &GraphQLContext,
        song_id: UUID,
        artist_id: Option<UUID>,
        album_id: Option<UUID>,
    ) -> errors::Result<StatsCollection> {
        let conn = context.connection();

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

    pub fn toggle_like(context: &GraphQLContext, song_id: &UUID) -> QueryResult<Song> {
        let conn = context.connection();

        diesel::update(song::table.filter(song::id.eq(song_id)))
            .set(song::liked.eq(not(song::liked)))
            .execute(conn)?;

        Song::from_id(context, &song_id)
    }
}

graphql_object!(Mutation: GraphQLContext |&self| {
    field play_song(
        &executor,
        song_id: UUID,
        artist_id: Option<UUID>,
        album_id: Option<UUID>,
    ) -> FieldResult<StatsCollection> {
        Ok(Mutation::play_song(executor.context(), song_id, artist_id, album_id)?)
    }

    field toggle_like(&executor, song_id: UUID) -> FieldResult<Song> {
        Ok(Mutation::toggle_like(executor.context(), &song_id)?)
    }
});
