use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;
use std::error::Error;

pub struct Artist {
    pub id: String,
    pub name: String,
    pub time_added: i32,
}

impl Artist {
    /*
    pub fn albums(context: GraphQLContext) -> Result<Vec<Album>, Box<Error>> {
    }
    */
}

graphql_object!(Artist: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field name() -> &str {
        &self.name
    }

/*
    field albums(&executor) -> FieldResult<Vec<Album>> {
        self.albums(executor.context())
    }
    */

    field time_added() -> i32 {
        self.time_added
    }
});
