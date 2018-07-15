use context::GraphQLContext;
use diesel::QueryResult;
use juniper::FieldResult;
use models::*;

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
        first: i64,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Artist>> {
        Artist::get_connection(context, first, after, sort)
    }

    pub fn song(context: &GraphQLContext, id: &UUID) -> QueryResult<Song> {
        Song::from_id(context, id)
    }

    pub fn songs(
        context: &GraphQLContext,
        first: i64,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Song>> {
        Song::get_connection(context, first, after, sort)
    }

    pub fn recently_added(context: &GraphQLContext, first: i64) -> FieldResult<Vec<RecentItem>> {
        RecentItem::recently_added(context, first)
    }

    pub fn recently_played(context: &GraphQLContext, first: i64) -> FieldResult<Vec<RecentItem>> {
        RecentItem::recently_played(context, first)
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
        Query::artists(executor.context(), first as i64, after, sort)
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
        Query::songs(executor.context(), first as i64, after, sort)
    }

    field recentlyAdded(
        &executor,
        first = 25: i32
    ) -> FieldResult<Vec<RecentItem>> {
        Query::recently_added(executor.context(), first as i64)
    }

    field recentlyPlayed(
        &executor,
        first = 25: i32
    ) -> FieldResult<Vec<RecentItem>> {
        Query::recently_played(executor.context(), first as i64)
    }
});
