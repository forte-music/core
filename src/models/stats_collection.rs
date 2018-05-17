use context::GraphQLContext;
use juniper::FieldResult;
use models::*;

pub struct StatsCollection {
    song_id: String,
    album_id: Option<String>,
    artist_id: Option<String>,
    playlist_id: Option<String>,
}

impl StatsCollection {
    pub fn song(&self, context: &GraphQLContext) -> FieldResult<Song> {
        Song::from_id(context, self.song_id.as_ref())
    }

    pub fn album_stats(&self, context: &GraphQLContext) -> FieldResult<Option<UserStats>> {
        if let Some(ref album_id) = self.album_id {
            let album = Album::from_id(context, album_id)?;
            let stats = album.stats();

            return Ok(Some(stats));
        }

        Ok(None)
    }

    pub fn artist_stats(&self, context: &GraphQLContext) -> FieldResult<Option<UserStats>> {
        if let Some(ref artist_id) = self.artist_id {
            let artist = Artist::from_id(context, artist_id)?;
            let stats = artist.stats();

            return Ok(Some(stats));
        }

        Ok(None)
    }

    pub fn playlist_stats(&self, context: &GraphQLContext) -> FieldResult<Option<UserStats>> {
        if let Some(ref playlist_id) = self.playlist_id {
            let playlist = Playlist::from_id(context, playlist_id)?;
            let stats = playlist.stats();

            return Ok(Some(stats));
        }

        Ok(None)
    }
}

graphql_object!(StatsCollection: GraphQLContext |&self| {
    field song(&executor) -> FieldResult<Song> {
        self.song(executor.context())
    }

    field album_stats(&executor) -> FieldResult<Option<UserStats>> {
        self.album_stats(executor.context())
    }

    field artist_stats(&executor) -> FieldResult<Option<UserStats>> {
        self.artist_stats(executor.context())
    }

    field playlist_stats(&executor) -> FieldResult<Option<UserStats>> {
        self.playlist_stats(executor.context())
    }
});
