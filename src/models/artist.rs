use database::album;
use database::artist;
use diesel::prelude::*;

use context::GraphQLContext;
use juniper::FieldResult;
use models::*;

#[derive(Queryable, Identifiable, Insertable, Clone)]
#[table_name = "artist"]
pub struct Artist {
    pub id: UUID,
    pub name: String,
    pub time_added: NaiveDateTime,

    pub last_played: Option<NaiveDateTime>,
}

impl Artist {
    pub fn from_id(context: &GraphQLContext, id: &UUID) -> QueryResult<Self> {
        let conn = context.connection();
        artist::table.filter(artist::id.eq(id)).first::<Self>(conn)
    }

    pub fn albums(&self, context: &GraphQLContext) -> QueryResult<Vec<Album>> {
        let conn = context.connection();
        Ok(album::table
            .filter(album::artist_id.eq(&self.id))
            .order(album::time_added.desc())
            .load::<Album>(conn)?)
    }

    pub fn stats(&self) -> UserStats {
        UserStats {
            id: format!("stats:{}", self.id.to_string()),
            last_played: self.last_played,
        }
    }
}

impl GetConnection<artist::table> for Artist {
    type Name = artist::name;
    type TimeAdded = artist::time_added;
    type LastPlayed = artist::last_played;

    fn name() -> Self::Name {
        artist::name
    }

    fn time_added() -> Self::TimeAdded {
        artist::time_added
    }

    fn last_played() -> Self::LastPlayed {
        artist::last_played
    }
}

graphql_object!(Artist: GraphQLContext |&self| {
    field id() -> &UUID {
        &self.id
    }

    field name() -> &str {
        &self.name
    }

    field albums(&executor) -> FieldResult<Vec<Album>> {
        Ok(self.albums(executor.context())?)
    }

    field stats() -> UserStats {
        self.stats()
    }

    field time_added() -> TimeWrapper {
        self.time_added.into()
    }
});
