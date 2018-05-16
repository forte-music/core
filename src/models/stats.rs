use context::GraphQLContext;
use juniper::ID;

#[derive(Queryable)]
pub struct SongUserStats {
    pub id: String,
    pub play_count: i32,
    pub last_played: Option<i32>,
    pub liked: bool,
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
