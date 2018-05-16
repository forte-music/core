use context::GraphQLContext;
use juniper::ID;

#[derive(Queryable)]
pub struct SongUserStats {
    pub id: String,
    pub play_count: i32,
    pub last_played: Option<i32>,
    pub liked: bool,
}

impl SongUserStats {
    pub fn gql_id(&self) -> ID {
        ID::from(self.id.to_owned())
    }
}

graphql_object!(SongUserStats: GraphQLContext |&self| {
    field id() -> ID {
        self.gql_id()
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
