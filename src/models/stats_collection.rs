use crate::context::GraphQLContext;
use crate::models::*;
use diesel::QueryResult;
use juniper::FieldResult;

pub struct StatsCollection {
    pub song_id: UUID,
    pub album_id: Option<UUID>,
    pub artist_id: Option<UUID>,
}

impl StatsCollection {
    pub fn song(&self, context: &GraphQLContext) -> QueryResult<Song> {
        Song::from_id(context, &self.song_id)
    }

    pub fn album_stats(&self, context: &GraphQLContext) -> QueryResult<Option<UserStats>> {
        if let Some(ref album_id) = self.album_id {
            let album = Album::from_id(context, album_id)?;
            let stats = album.stats();

            return Ok(Some(stats));
        }

        Ok(None)
    }

    pub fn artist_stats(&self, context: &GraphQLContext) -> QueryResult<Option<UserStats>> {
        if let Some(ref artist_id) = self.artist_id {
            let artist = Artist::from_id(context, artist_id)?;
            let stats = artist.stats();

            return Ok(Some(stats));
        }

        Ok(None)
    }
}

graphql_object!(StatsCollection: GraphQLContext |&self| {
    field song(&executor) -> FieldResult<Song> {
        Ok(self.song(executor.context())?)
    }

    field album_stats(&executor) -> FieldResult<Option<UserStats>> {
        Ok(self.album_stats(executor.context())?)
    }

    field artist_stats(&executor) -> FieldResult<Option<UserStats>> {
        Ok(self.artist_stats(executor.context())?)
    }
});
