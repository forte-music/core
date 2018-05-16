use database::song;
use diesel::prelude::*;

use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;

#[derive(Queryable)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub album_id: String,
    pub stat_id: String,
    pub track_number: i32,
    pub disk_number: i32,
    pub duration: i32,
    pub time_added: i32,
}

impl Song {
    pub fn from_id(context: &GraphQLContext, id: &str) -> FieldResult<Self> {
        let conn = &*context.connection;
        Ok(song::table.filter(song::id.eq(id)).first::<Self>(conn)?)
    }

    pub fn album(&self, context: &GraphQLContext) -> FieldResult<Album> {
        Album::from_id(context, self.album_id.as_str())
    }

    pub fn artists(&self, context: &GraphQLContext) -> FieldResult<Vec<Artist>> {
        NotImplementedErr()
    }

    pub fn stats(&self, context: &GraphQLContext) -> FieldResult<SongUserStats> {
        NotImplementedErr()
    }
}

graphql_object!(Song: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field name() -> &str {
        &self.name
    }

    field album(&executor) -> FieldResult<Album> {
        self.album(executor.context())
    }

    field artists(&executor) -> FieldResult<Vec<Artist>> {
        self.artists(executor.context())
    }

    field stream_url() -> &str {
    // TODO: Remove Me
        "test"
    }

    field track_number() -> i32 {
        self.track_number
    }

    field disk_number() -> i32 {
        self.disk_number
    }

    field stats(&executor) -> FieldResult<SongUserStats> {
        self.stats(executor.context())
    }

    field duration() -> i32 {
        self.duration
    }

    field time_added() -> i32 {
        self.time_added
    }
});
