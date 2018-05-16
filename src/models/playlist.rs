use database::playlist;
use database::playlist_item;
use database::song;
use diesel::prelude::*;

use context::GraphQLContext;
use diesel::dsl;
use juniper::{FieldResult, ID};
use models::Connection;
use models::*;

#[derive(Queryable)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub time_added: i32,
}

impl Playlist {
    pub fn from_id(context: &GraphQLContext, id: &str) -> FieldResult<Self> {
        let conn = &*context.connection;
        Ok(playlist::table
            .filter(playlist::id.eq(id))
            .first::<Self>(conn)?)
    }

    pub fn duration(&self, context: &GraphQLContext) -> FieldResult<i32> {
        let conn = &*context.connection;
        let maybe_duration: Option<i64> = playlist_item::table
            .filter(playlist_item::playlist_id.eq(self.id.as_str()))
            .inner_join(song::table)
            .select(dsl::sum(song::duration))
            .first::<Option<i64>>(conn)?;

        let duration = maybe_duration.unwrap_or(0);

        Ok(duration as i32)
    }

    pub fn items(
        &self,
        context: &GraphQLContext,
        input: &ConnectionQuery,
    ) -> FieldResult<Connection<PlaylistItem>> {
        NotImplementedErr()
        // TODO: Handle Ordering
        // let conn = &*context.connection;
        // Ok(playlist_item::table
        //     .filter(playlist_item::playlist_id.eq(self.id.as_ref()))
        //     .load::<PlaylistItem>(conn)?)
    }
}

pub struct PlaylistItem {
    pub id: String,
    pub rank: String,
    pub song_id: String,
}

impl PlaylistItem {
    pub fn song(&self, context: &GraphQLContext) -> FieldResult<Song> {
        Song::from_id(context, self.id.as_str())
    }
}

#[derive(GraphQLInputObject)]
pub struct PlaylistAppendInput {
    pub playlist_id: ID,
    pub songs: Vec<ID>,
}

#[derive(GraphQLEnum)]
pub enum Position {
    #[graphql(name = "BEGINNING")]
    Beginning,
    #[graphql(name = "END")]
    End,
}

#[derive(GraphQLEnum)]
pub enum Offset {
    #[graphql(name = "AFTER")]
    After,
    #[graphql(name = "BEFORE")]
    Before,
}

graphql_object!(Playlist: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field name() -> &str {
        &self.name
    }

    field duration(&executor) -> FieldResult<i32> {
        self.duration(executor.context())
    }

    field time_added() -> i32 {
        self.time_added
    }

    field items(&executor, input: ConnectionQuery) -> FieldResult<Connection<PlaylistItem>> {
        self.items(executor.context(), &input)
    }
});

graphql_object!(PlaylistItem: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field song(&executor) -> FieldResult<Song> {
        self.song(executor.context())
    }
});
