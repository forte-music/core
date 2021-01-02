use crate::context::GraphQLContext;
use crate::database::artist;
use crate::database::song;
use crate::database::song_artist;
use crate::models::*;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

#[derive(Queryable, Identifiable, Insertable)]
#[table_name = "song"]
pub struct Song {
    pub id: UUID,
    pub name: String,
    pub album_id: UUID,
    pub track_number: i32,
    pub disk_number: i32,

    pub duration: i32,
    pub time_added: NaiveDateTime,

    pub play_count: i32,
    pub last_played: Option<NaiveDateTime>,
    pub liked: bool,
    pub path: PathWrapper,
}

impl Song {
    pub fn from_id(conn: &SqliteConnection, id: UUID) -> QueryResult<Self> {
        song::table.find(id).first::<Self>(conn)
    }

    pub fn get_raw_stream_url(id: &str) -> String {
        format!("/files/music/{}/raw", id)
    }
}

impl GetConnection<song::table> for Song {
    type Name = song::name;
    type TimeAdded = song::time_added;
    type LastPlayed = song::last_played;

    fn name() -> Self::Name {
        song::name
    }

    fn time_added() -> Self::TimeAdded {
        song::time_added
    }

    fn last_played() -> Self::LastPlayed {
        song::last_played
    }
}

#[juniper::graphql_object(context = GraphQLContext)]
impl Song {
    fn id(&self) -> UUID {
        self.id
    }

    fn stream_url(&self) -> String {
        Song::get_raw_stream_url(&self.id.to_string())
    }

    fn track_number(&self) -> i32 {
        self.track_number
    }

    fn disk_number(&self) -> i32 {
        self.disk_number
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn album(&self, context: &GraphQLContext) -> FieldResult<Album> {
        Album::from_id(&context.connection(), self.album_id).map_err(FieldError::from)
    }

    fn artists(&self, context: &GraphQLContext) -> FieldResult<Vec<Artist>> {
        let conn = &context.connection() as &SqliteConnection;
        song_artist::table
            .filter(song_artist::song_id.eq(&self.id))
            .inner_join(artist::table)
            .select(artist::all_columns)
            .load::<Artist>(conn)
            .map_err(FieldError::from)
    }

    fn stats(&self) -> UserStats {
        UserStats {
            id: format!("stats:{}", self.id.to_string()),
            last_played: self.last_played,
        }
    }

    fn song_stats(&self) -> SongUserStats {
        SongUserStats {
            id: format!("song_stats:{}", self.id.to_string()),
            play_count: self.play_count,
            liked: self.liked,
        }
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn time_added(&self) -> TimeWrapper {
        self.time_added.into()
    }
}
