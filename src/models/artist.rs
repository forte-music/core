use database::album;
use database::artist;
use diesel::prelude::*;

use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;

#[derive(Queryable)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub time_added: i32,
}

impl Artist {
    pub fn from_id(context: &GraphQLContext, id: &str) -> FieldResult<Self> {
        let conn = &*context.connection;
        Ok(artist::table.filter(artist::id.eq(id)).first::<Self>(conn)?)
    }

    pub fn gql_id(&self) -> ID {
        ID::from(self.id.to_owned())
    }

    pub fn albums(&self, context: &GraphQLContext) -> FieldResult<Vec<Album>> {
        let conn = &*context.connection;
        Ok(album::table
            .filter(album::artist_id.eq(self.id.as_str()))
            .order(album::time_added.desc())
            .load::<Album>(conn)?)
    }

    pub fn stats(&self, context: &GraphQLContext) -> FieldResult<UserStats> {
        NotImplementedErr()
    }
}

graphql_object!(Artist: GraphQLContext |&self| {
    field id() -> ID {
        self.gql_id()
    }

    field name() -> &str {
        &self.name
    }

    field albums(&executor) -> FieldResult<Vec<Album>> {
        self.albums(executor.context())
    }

    field stats(&executor) -> FieldResult<UserStats> {
        self.stats(executor.context())
    }

    field time_added() -> i32 {
        self.time_added
    }
});
