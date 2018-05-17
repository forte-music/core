use juniper::FieldResult;
use context::GraphQLContext;
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
        NotImplementedErr()
    }

    pub fn artist_stats(&self, context: &GraphQLContext) -> FieldResult<Option<UserStats>> {
        NotImplementedErr()
    }

    pub fn playlist_stats(&self, context: &GraphQLContext) -> FieldResult<Option<UserStats>> {
        NotImplementedErr()
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
