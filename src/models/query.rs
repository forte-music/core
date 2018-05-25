use context::GraphQLContext;
use diesel::QueryResult;
use juniper::FieldResult;
use models::*;

use models::Connection;

pub struct Query;

impl Query {
    pub fn album(context: &GraphQLContext, id: &UUID) -> QueryResult<Album> {
        Album::from_id(context, id)
    }

    pub fn albums(
        context: &GraphQLContext,
        first: i64,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Album>> {
        Album::get_connection(context, first, after, sort)
    }

    pub fn artist(context: &GraphQLContext, id: &UUID) -> QueryResult<Artist> {
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

    pub fn song(context: &GraphQLContext, id: &UUID) -> QueryResult<Song> {
        Song::from_id(context, id)
    }

    pub fn songs(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Song>> {
        NotImplementedErr()
    }

    pub fn playlist(context: &GraphQLContext, id: &UUID) -> QueryResult<Playlist> {
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
        Ok(Query::album(executor.context(), &id)?)
    }

    field albums(
        &executor,
        first = 25: i32,
        after: Option<String>,
        sort: Option<SortParams>
    ) -> FieldResult<Connection<Album>> {
        Query::albums(executor.context(), first as i64, after, sort)
    }

    field artist(&executor, id: UUID) -> FieldResult<Artist> {
        Ok(Query::artist(executor.context(), &id)?)
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
        Ok(Query::song(executor.context(), &id)?)
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
        Ok(Query::playlist(executor.context(), &id)?)
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
