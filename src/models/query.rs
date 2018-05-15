use context::GraphQLContext;

pub struct Query;

impl Query {}

graphql_object!(
    Query: GraphQLContext | &self | {
        /*
    field album(&executor, id: ID) -> FieldResult<Album> {
        Query::album(executor.context(), &id)
    }

    field albums(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Album>> {
        Query::albums(executor.context())
    }

    field artist(&executor, id: ID) -> FieldResult<Artist> {
        Query::artist(executor.context(), &id)
    }

    field artists(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Artist>> {
        Query::artists(executor.context())
    }

    field song(&executor, id: ID) -> FieldResult<Song> {
        Query::song(executor.context(), &id)
    }

    field songs(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Song>> {
        Query::songs(executor.context())
    }

    field playlist(&executor, id: ID) -> FieldResult<Playlist> {
        Query::playlist(executor.context(), &id)
    }

    field playlists(&executor, input: ConnectionQuery, sort: Option<SortParams>)
            -> FieldResult<Connection<Playlist>> {
        Query::playlists(executor.context())
    }
    */
    }
);
