use database::album;
use diesel::prelude::*;
use juniper::{FieldResult, ID};

use context::GraphQLContext;
use models::*;

#[derive(Queryable)]
pub struct Album {
    pub id: String,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist_id: String,
    pub release_year: i32,
    pub time_added: i32,
}

impl Album {
    pub fn from_id(context: &GraphQLContext, id: &str) -> FieldResult<Self> {
        let conn = &*context.connection;

        Ok(album::table.filter(album::id.eq(id)).first::<Album>(conn)?)
    }

    pub fn artist(&self, context: &GraphQLContext) -> FieldResult<Artist> {
        NotImplementedErr()
    }

    pub fn songs(&self, context: &GraphQLContext) -> FieldResult<Vec<Song>> {
        NotImplementedErr()
    }

    pub fn duration(&self, context: &GraphQLContext) -> FieldResult<i32> {
        NotImplementedErr()
    }
}

graphql_object!(Album: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field artwork_url() -> &Option<String> {
        &self.artwork_url
    }

    field name() -> &str {
        &self.name
    }

    field artist(&executor) -> FieldResult<Artist> {
        self.artist(executor.context())
    }

    field songs(&executor) -> FieldResult<Vec<Song>> {
        self.songs(executor.context())
    }

    field duration(&executor) -> FieldResult<i32> {
        self.duration(executor.context())
    }

    field release_year() -> i32 {
        self.release_year
    }

    field time_added() -> i32 {
        self.time_added
    }
});
