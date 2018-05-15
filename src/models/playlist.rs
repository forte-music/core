use context::GraphQLContext;
use juniper::{FieldResult, ID};

pub struct Playlist {
    pub id: String,
    pub name: String,
    pub time_added: i32,
}

pub struct PlaylistItem {
    pub id: String,
    pub rank: String,
    pub song_id: String,
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

/*
    field duration(&executor) -> FieldResult<i32> {
        self.duration(executor.context())
    }
    */

    field time_added() -> i32 {
        self.time_added
    }

/*
    field items(&executor, input: ConnectionQuery) -> FieldResult<Connection<PlaylistItem>> {
        self.items(&input, executor.context())
    }
    */
});

graphql_object!(PlaylistItem: GraphQLContext |&self| {
    field id() -> ID {
        ID::from(self.id.clone())
    }

/*
    field song(&executor) -> FieldResult<Song> {
        self.song(executor.context())
    }
    */
});
