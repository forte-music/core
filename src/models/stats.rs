use database::song_user_stats;
use diesel::prelude::*;

use context::GraphQLContext;
use juniper::FieldResult;
use juniper::ID;

#[derive(Queryable)]
pub struct SongUserStats {
    pub id: String,
    pub play_count: i32,
    pub last_played: Option<i32>,
    pub liked: bool,
}

impl SongUserStats {
    pub fn from_id(context: &GraphQLContext, id: &str) -> FieldResult<Self> {
        let conn = &*context.connection;
        Ok(song_user_stats::table
            .filter(song_user_stats::id.eq(id))
            .first::<Self>(conn)?)
    }
}

graphql_object!(SongUserStats: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field play_count() -> i32 {
        self.play_count
    }

    field last_played() -> Option<i32> {
        self.last_played
    }

    field liked() -> bool {
        self.liked
    }
});
