use crate::context::GraphQLContext;
use crate::database::album;
use crate::database::artist;
use crate::models::*;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

#[derive(Queryable, Identifiable, Insertable, Clone)]
#[table_name = "artist"]
pub struct Artist {
    pub id: UUID,
    pub name: String,
    pub time_added: NaiveDateTime,
    pub last_played: Option<NaiveDateTime>,
}

impl Artist {
    pub fn from_id(conn: &SqliteConnection, id: UUID) -> QueryResult<Self> {
        artist::table.find(id).first::<Self>(conn)
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

#[graphql_object(context = GraphQLContext)]
impl Artist {
    fn id(&self) -> UUID {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn albums(&self, context: &GraphQLContext) -> FieldResult<Vec<Album>> {
        let conn = &context.connection() as &SqliteConnection;
        album::table
            .filter(album::artist_id.eq(&self.id))
            .order(album::time_added.desc())
            .load::<Album>(conn)
            .map_err(FieldError::from)
    }

    fn stats(&self) -> UserStats {
        self.stats()
    }

    fn time_added(&self) -> TimeWrapper {
        self.time_added.into()
    }
}
