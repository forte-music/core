use database::artist;
use database::song;
use database::song_artist;
use diesel::prelude::*;

use context::GraphQLContext;
use juniper::FieldResult;
use models::*;

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
    pub fn from_id(context: &GraphQLContext, id: &UUID) -> QueryResult<Self> {
        let conn = context.connection();
        song::table.filter(song::id.eq(id)).first::<Self>(conn)
    }

    pub fn get_stream_url_for_id<T: AsRef<str>>(id: T) -> String {
        format!("/files/music/{}/raw", id)
    }

    pub fn stream_url(&self) -> String {
        Song::get_stream_url_for_id(self.id.to_string())
    }

    pub fn album(&self, context: &GraphQLContext) -> QueryResult<Album> {
        Album::from_id(context, &self.album_id)
    }

    pub fn artists(&self, context: &GraphQLContext) -> QueryResult<Vec<Artist>> {
        let conn = context.connection();
        Ok(song_artist::table
            .filter(song_artist::song_id.eq(&self.id))
            .inner_join(artist::table)
            .select(artist::all_columns)
            .load::<Artist>(conn)?)
    }

    pub fn stats(&self) -> UserStats {
        UserStats {
            id: format!("stats:{}", self.id.to_string()),
            last_played: self.last_played,
        }
    }

    pub fn song_stats(&self) -> SongUserStats {
        SongUserStats {
            id: format!("song_stats:{}", self.id.to_string()),
            play_count: self.play_count,
            liked: self.liked,
        }
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

graphql_object!(Song: GraphQLContext |&self| {
    field id() -> &UUID {
        &self.id
    }

    field stream_url() -> String {
        self.stream_url()
    }

    field track_number() -> i32 {
        self.track_number as i32
    }

    field disk_number() -> i32 {
        self.disk_number
    }

    field name() -> &str {
        &self.name
    }

    field album(&executor) -> FieldResult<Album> {
        Ok(self.album(executor.context())?)
    }

    field artists(&executor) -> FieldResult<Vec<Artist>> {
        Ok(self.artists(executor.context())?)
    }

    field stats() -> UserStats {
        self.stats()
    }

    field song_stats() -> SongUserStats {
        self.song_stats()
    }

    field duration() -> i32 {
        self.duration
    }

    field time_added() -> TimeWrapper {
        self.time_added.into()
    }
});
