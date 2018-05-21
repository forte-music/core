use context::GraphQLContext;
use juniper::ID;

pub struct UserStats {
    pub id: String,
    pub last_played: Option<i32>,
}

impl UserStats {
    pub fn gql_id(&self) -> ID {
        ID::from(self.id.to_owned())
    }
}

graphql_object!(UserStats: GraphQLContext |&self| {
    field id() -> ID {
        self.gql_id()
    }

    field last_played() -> Option<i32> {
        self.last_played
    }
});
