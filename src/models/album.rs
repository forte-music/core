use database::album;
use database::song;
use diesel::prelude::*;
use juniper::{FieldResult, ID};

use context::GraphQLContext;
use diesel::dsl;
use models::*;

#[derive(Queryable)]
pub struct Album {
    pub id: String,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist_id: String,
    pub release_year: i32,
    pub time_added: i32,

    pub last_played: Option<i32>,
}

impl Album {
    pub fn from_id(context: &GraphQLContext, id: &str) -> FieldResult<Self> {
        let conn = context.connection();
        Ok(album::table.filter(album::id.eq(id)).first::<Self>(conn)?)
    }

    pub fn gql_id(&self) -> ID {
        ID::from(self.id.to_owned())
    }

    pub fn artist(&self, context: &GraphQLContext) -> FieldResult<Artist> {
        Artist::from_id(context, self.artist_id.as_str())
    }

    pub fn songs(&self, context: &GraphQLContext) -> FieldResult<Vec<Song>> {
        let conn = context.connection();
        Ok(song::table
            .filter(song::album_id.eq(self.id.as_str()))
            .order(song::time_added.desc())
            .load::<Song>(conn)?)
    }

    pub fn duration(&self, context: &GraphQLContext) -> FieldResult<i32> {
        let conn = context.connection();
        let maybe_duration: Option<i64> = song::table
            .select(dsl::sum(song::duration))
            .filter(song::album_id.eq(self.id.as_str()))
            .first::<Option<i64>>(conn)?;
        let duration = maybe_duration.unwrap_or(0);

        Ok(duration as i32)
    }

    pub fn stats(&self) -> UserStats {
        UserStats {
            id: format!("stats:{}", self.id),
            last_played: self.last_played,
        }
    }
}

impl GetConnection<album::table> for Album {
    type Name = album::name;
    type TimeAdded = album::time_added;
    type LastPlayed = album::last_played;

    fn table() -> album::table {
        album::table
    }

    fn name() -> Self::Name {
        album::name
    }

    fn time_added() -> Self::TimeAdded {
        album::time_added
    }

    fn last_played() -> Self::LastPlayed {
        album::last_played
    }
}

graphql_object!(Album: GraphQLContext |&self| {
    field id() -> ID {
        self.gql_id()
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

    field stats() -> UserStats {
        self.stats()
    }

    field time_added() -> i32 {
        self.time_added
    }
});
