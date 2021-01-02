use crate::context::GraphQLContext;
use crate::models::*;
use juniper::{FieldError, FieldResult};

pub struct Query;

#[juniper::graphql_object(context = GraphQLContext)]
impl Query {
    fn album(context: &GraphQLContext, id: UUID) -> FieldResult<Album> {
        Album::from_id(&context.connection(), id).map_err(FieldError::from)
    }

    #[graphql(arguments(first(default = 25)))]
    fn albums(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Album>> {
        Album::get_connection(context, first as i64, after, sort)
    }

    fn artist(context: &GraphQLContext, id: UUID) -> FieldResult<Artist> {
        Artist::from_id(&context.connection(), id).map_err(FieldError::from)
    }

    #[graphql(arguments(first(default = 25)))]
    fn artists(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Artist>> {
        Artist::get_connection(context, first as i64, after, sort)
    }

    fn song(context: &GraphQLContext, id: UUID) -> FieldResult<Song> {
        Song::from_id(&context.connection(), id).map_err(FieldError::from)
    }

    #[graphql(arguments(first(default = 25)))]
    fn songs(
        context: &GraphQLContext,
        first: i32,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Song>> {
        Song::get_connection(context, first as i64, after, sort)
    }

    #[graphql(arguments(first(default = 25)))]
    fn recently_added(context: &GraphQLContext, first: i32) -> FieldResult<Vec<RecentItem>> {
        RecentItem::recently_added(context, first as i64)
    }

    #[graphql(arguments(first(default = 25)))]
    fn recently_played(context: &GraphQLContext, first: i32) -> FieldResult<Vec<RecentItem>> {
        RecentItem::recently_played(context, first as i64)
    }
}
