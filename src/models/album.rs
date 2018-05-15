use std::error::Error;
// use database::album;
use juniper::{FieldResult, ID};

use context::GraphQLContext;
use models::*;

pub struct Album {
    pub id: String,
    pub artwork_url: Option<String>,
    pub name: String,
    pub artist_id: String,
    pub release_year: i32,
    pub time_added: i32,
}

impl Album {
    /*
    pub fn from_id(context: GraphQLContext, id: &str) -> Result<Self, Box<Error>> {
    }

    pub fn artist(context: GraphQLContext) -> Result<Artist, Box<Error>> {
    }

    pub fn songs(context: GraphQLContext) -> Result<Vec<Song>, Box<Error>> {
    }

    pub fn duration(context: GraphQLContext) -> Result<i32, Box<Error>> {
    }
    */
}

graphql_object!(Album: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

    field artwork_url() -> &Option<String> {
        &self.artwork_url
    }

    field name() -> &str {
        &self.name
    }

/*
    field artist(&executor) -> FieldResult<Artist> {
        self.artist(executor.context())
    }

    field songs(&executor) -> FieldResult<Vec<Song>> {
        self.songs(executor.context())
    }

    field duration(&executor) -> FieldResult<i32> {
        self.duration(executor.context())
    }
    */

    field release_year() -> i32 {
        self.release_year
    }

    field time_added() -> i32 {
        self.time_added
    }
});
