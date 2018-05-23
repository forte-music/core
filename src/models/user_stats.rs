use context::GraphQLContext;
use juniper::ID;
use models::*;

pub struct UserStats {
    pub id: String,
    pub last_played: Option<NaiveDateTime>,
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

    field last_played() -> Option<TimeWrapper> {
        self.last_played.map(|t| t.into())
    }
});
