use database::album;
use database::song;
use diesel::prelude::*;
use juniper::FieldResult;

use context::GraphQLContext;
use diesel::dsl;
use models::*;

#[derive(Queryable, Identifiable, Insertable, Clone)]
#[table_name = "album"]
pub struct Album {
    pub id: UUID,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist_id: UUID,
    pub release_year: Option<i32>,
    pub time_added: NaiveDateTime,

    pub last_played: Option<NaiveDateTime>,
}

impl Album {
    pub fn from_id(context: &GraphQLContext, id: &UUID) -> FieldResult<Self> {
        let conn = context.connection();
        Ok(album::table.filter(album::id.eq(id)).first::<Self>(conn)?)
    }

    pub fn artist(&self, context: &GraphQLContext) -> FieldResult<Artist> {
        Artist::from_id(context, &self.artist_id)
    }

    pub fn songs(&self, context: &GraphQLContext) -> FieldResult<Vec<Song>> {
        let conn = context.connection();
        Ok(song::table
            .filter(song::album_id.eq(&self.id))
            .order(song::time_added.desc())
            .load::<Song>(conn)?)
    }

    pub fn duration(&self, context: &GraphQLContext) -> FieldResult<i32> {
        let conn = context.connection();
        let maybe_duration: Option<i64> = song::table
            .filter(song::album_id.eq(&self.id))
            .select(dsl::sum(song::duration))
            .first::<Option<i64>>(conn)?;
        let duration = maybe_duration.unwrap_or(0);

        Ok(duration as i32)
    }

    pub fn stats(&self) -> UserStats {
        UserStats {
            id: format!("stats:{}", self.id.to_string()),
            last_played: self.last_played,
        }
    }
}

graphql_object!(Album: GraphQLContext |&self| {
    field id() -> &UUID {
        &self.id
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

    field release_year() -> Option<i32> {
        self.release_year
    }

    field stats() -> UserStats {
        self.stats()
    }

    field time_added() -> TimeWrapper {
        self.time_added.into()
    }
});
