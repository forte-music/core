use crate::context::GraphQLContext;
use crate::models::*;
use juniper::{FieldError, FieldResult};

pub struct StatsCollection {
    pub song_id: UUID,
    pub album_id: Option<UUID>,
    pub artist_id: Option<UUID>,
}

#[graphql_object(context = GraphQLContext)]
impl StatsCollection {
    fn song(&self, context: &GraphQLContext) -> FieldResult<Song> {
        Song::from_id(context, self.song_id).map_err(FieldError::from)
    }

    fn album_stats(&self, context: &GraphQLContext) -> FieldResult<Option<UserStats>> {
        if let Some(album_id) = self.album_id {
            let album = Album::from_id(context, album_id)?;
            let stats = album.stats();

            return Ok(Some(stats));
        }

        Ok(None)
    }

    fn artist_stats(&self, context: &GraphQLContext) -> FieldResult<Option<UserStats>> {
        if let Some(artist_id) = self.artist_id {
            let artist = Artist::from_id(context, artist_id)?;
            let stats = artist.stats();

            return Ok(Some(stats));
        }

        Ok(None)
    }
}
