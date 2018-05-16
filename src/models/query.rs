use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;

pub struct Query;

impl Query {
    pub fn album(context: &GraphQLContext, id: &str) -> FieldResult<Album> {
        Album::from_id(context, id)
    }

    pub fn albums(
        context: &GraphQLContext,
        input: ConnectionQuery,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Album>> {
        NotImplementedErr()
    }

    pub fn artist(context: &GraphQLContext, id: &str) -> FieldResult<Artist> {
        Artist::from_id(context, id)
    }

    pub fn artists(
        context: &GraphQLContext,
        input: ConnectionQuery,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Artist>> {
        NotImplementedErr()
    }

    pub fn song(context: &GraphQLContext, id: &str) -> FieldResult<Song> {
        Song::from_id(context, id)
    }

    pub fn songs(
        context: &GraphQLContext,
        input: ConnectionQuery,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Song>> {
        NotImplementedErr()
    }

    pub fn playlist(context: &GraphQLContext, id: &str) -> FieldResult<Playlist> {
        Playlist::from_id(context, id)
    }

    pub fn playlists(
        context: &GraphQLContext,
        input: ConnectionQuery,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Playlist>> {
        NotImplementedErr()
    }
}

graphql_object!(
    Query: GraphQLContext | &self | {
    field album(&executor, id: ID) -> FieldResult<Album> {
        Query::album(executor.context(), &id)
    }

    field albums(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Album>> {
        Query::albums(executor.context(), input, sort)
    }

    field artist(&executor, id: ID) -> FieldResult<Artist> {
        Query::artist(executor.context(), &id)
    }

    field artists(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Artist>> {
        Query::artists(executor.context(), input, sort)
    }

    field song(&executor, id: ID) -> FieldResult<Song> {
        Query::song(executor.context(), &id)
    }

    field songs(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Song>> {
        Query::songs(executor.context(), input, sort)
    }

    field playlist(&executor, id: ID) -> FieldResult<Playlist> {
        Query::playlist(executor.context(), &id)
    }

    field playlists(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Playlist>> {
        Query::playlists(executor.context(), input, sort)
    }
    }
);
