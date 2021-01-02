use crate::context::GraphQLContext;
use crate::database::album;
use crate::database::song;
use crate::models::*;
use diesel::dsl;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

#[derive(Queryable, Identifiable, Insertable, Clone)]
#[table_name = "album"]
pub struct Album {
    pub id: UUID,
    pub artwork_path: Option<PathWrapper>,
    pub name: String,
    pub artist_id: UUID,
    pub release_year: Option<i32>,
    pub time_added: NaiveDateTime,
    pub last_played: Option<NaiveDateTime>,
}

impl Album {
    pub fn from_id(context: &GraphQLContext, id: UUID) -> QueryResult<Self> {
        let conn = &context.connection() as &SqliteConnection;
        album::table.filter(album::id.eq(id)).first::<Self>(conn)
    }

    pub fn get_artwork_url(id: &str) -> String {
        format!("/files/artwork/{}/raw", id)
    }

    pub fn stats(&self) -> UserStats {
        UserStats {
            id: format!("stats:{}", self.id.to_string()),
            last_played: self.last_played,
        }
    }
}

impl GetConnection<album::table> for Album {
    type Name = album::name;
    type TimeAdded = album::time_added;
    type LastPlayed = album::last_played;

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

#[graphql_object(context = GraphQLContext)]
impl Album {
    fn id(&self) -> UUID {
        self.id
    }

    fn artwork_url(&self) -> Option<String> {
        match self.artwork_path {
            Some(_) => Some(Album::get_artwork_url(&self.id.to_string())),
            _ => None,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn artist(&self, context: &GraphQLContext) -> FieldResult<Artist> {
        Artist::from_id(context, self.artist_id).map_err(FieldError::from)
    }

    fn songs(&self, context: &GraphQLContext) -> FieldResult<Vec<Song>> {
        let conn = &context.connection() as &SqliteConnection;
        song::table
            .filter(song::album_id.eq(&self.id))
            .order_by(song::disk_number.asc())
            .then_order_by(song::track_number.asc())
            .load::<Song>(conn)
            .map_err(FieldError::from)
    }

    fn duration(&self, context: &GraphQLContext) -> FieldResult<i32> {
        let conn = &context.connection() as &SqliteConnection;
        let maybe_duration: Option<i64> = song::table
            .filter(song::album_id.eq(&self.id))
            .select(dsl::sum(song::duration))
            .first::<Option<i64>>(conn)?;
        let duration = maybe_duration.unwrap_or(0);

        Ok(duration as i32)
    }

    fn release_year(&self) -> Option<i32> {
        self.release_year
    }

    fn stats(&self) -> UserStats {
        self.stats()
    }

    fn time_added(&self) -> TimeWrapper {
        self.time_added.into()
    }
}
