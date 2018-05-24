use context::GraphQLContext;
use juniper::FieldResult;
use models::*;

pub struct Query;

impl Query {
    pub fn album(context: &GraphQLContext, id: &UUID) -> FieldResult<Album> {
        Album::from_id(context, id)
    }

    pub fn albums(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Album>> {
        NotImplementedErr()
    }

    pub fn artist(context: &GraphQLContext, id: &UUID) -> FieldResult<Artist> {
        Artist::from_id(context, id)
    }

    pub fn artists(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Artist>> {
        NotImplementedErr()
    }

    pub fn song(context: &GraphQLContext, id: &UUID) -> FieldResult<Song> {
        Ok(Song::from_id(context, id)?)
    }

    pub fn songs(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Song>> {
        NotImplementedErr()
    }

    pub fn playlist(context: &GraphQLContext, id: &UUID) -> FieldResult<Playlist> {
        Playlist::from_id(context, id)
    }

    pub fn playlists(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Playlist>> {
        NotImplementedErr()
    }
}

graphql_object!(Query: GraphQLContext |&self| {
    field album(&executor, id: UUID) -> FieldResult<Album> {
        Query::album(executor.context(), &id)
    }

    field albums(
        &executor,
        first = 25: i32,
        after: Option<String>,
        sort: Option<SortParams>
    ) -> FieldResult<Connection<Album>> {
        Query::albums(executor.context(), first, after, sort)
    }

    field artist(&executor, id: UUID) -> FieldResult<Artist> {
        Query::artist(executor.context(), &id)
    }

    field artists(
        &executor,
        first = 25: i32,
        after: Option<String>,
        sort: Option<SortParams>
    ) -> FieldResult<Connection<Artist>> {
        Query::artists(executor.context(), first, after, sort)
    }

    field song(&executor, id: UUID) -> FieldResult<Song> {
        Query::song(executor.context(), &id)
    }

    field songs(
        &executor,
        first = 25: i32,
        after: Option<String>,
        sort: Option<SortParams>
    ) -> FieldResult<Connection<Song>> {
        Query::songs(executor.context(), first, after, sort)
    }

    field playlist(&executor, id: UUID) -> FieldResult<Playlist> {
        Query::playlist(executor.context(), &id)
    }

    field playlists(
        &executor,
        first = 25: i32,
        after: Option<String>,
        sort: Option<SortParams>
    ) -> FieldResult<Connection<Playlist>> {
        Query::playlists(executor.context(), first, after, sort)
    }
});
