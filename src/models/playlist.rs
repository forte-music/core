use database::playlist;
use database::playlist_item;
use database::song;
use diesel::dsl;
use diesel::prelude::*;

use juniper::FieldResult;

use context::GraphQLContext;
use models::Connection;
use models::*;

#[derive(Queryable, Identifiable, Insertable)]
#[table_name = "playlist"]
pub struct Playlist {
    pub id: UUID,
    pub name: String,
    pub description: String,
    pub time_added: i32,

    pub last_played: Option<i32>,
}

impl Playlist {
    pub fn from_id(context: &GraphQLContext, id: &UUID) -> FieldResult<Self> {
        let conn = context.connection();
        Ok(playlist::table
            .filter(playlist::id.eq(id))
            .first::<Self>(conn)?)
    }

    pub fn duration(&self, context: &GraphQLContext) -> FieldResult<i32> {
        let conn = context.connection();
        let maybe_duration: Option<i64> = playlist_item::table
            .filter(playlist_item::playlist_id.eq(&self.id))
            .inner_join(song::table)
            .select(dsl::sum(song::duration))
            .first::<Option<i64>>(conn)?;

        let duration = maybe_duration.unwrap_or(0);

        Ok(duration as i32)
    }

    pub fn items(
        &self,
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<PlaylistItem>> {
        NotImplementedErr()
    }

    pub fn stats(&self) -> UserStats {
        UserStats {
            id: format!("stats:{}", self.id.to_string()),
            last_played: self.last_played,
        }
    }
}

pub struct PlaylistItem {
    pub id: UUID,
    pub rank: String,
    pub song_id: UUID,
}

impl PlaylistItem {
    pub fn song(&self, context: &GraphQLContext) -> FieldResult<Song> {
        Song::from_id(context, &self.song_id)
    }
}

graphql_object!(Playlist: GraphQLContext |&self| {
    field id() -> &UUID {
        &self.id
    }

    field name() -> &str {
        &self.name
    }

    field description() -> &str {
        &self.description
    }

    field duration(&executor) -> FieldResult<i32> {
        self.duration(executor.context())
    }

    field items(
        &executor,
        first = 25: i32,
        after: Option<String>,
        sort: Option<SortParams>
    ) -> FieldResult<Connection<PlaylistItem>> {
        self.items(executor.context(), first, after, sort)
    }

    field stats() -> UserStats {
        self.stats()
    }

    field time_added() -> i32 {
        self.time_added
    }
});

graphql_object!(PlaylistItem: GraphQLContext |&self| {
    field id() -> &UUID {
        &self.id
    }

    field song(&executor) -> FieldResult<Song> {
        self.song(executor.context())
    }
});
