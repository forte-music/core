use context::GraphQLContext;
use juniper::FieldResult;
use models::*;

use diesel;
use diesel::dsl;
use diesel::prelude::*;
use diesel::query_builder::BoxedSelectStatement;
use diesel::sql_types::Integer;
use diesel::sql_types::Nullable;
use diesel::sql_types::Text;

use database::album;
use diesel::backend::Backend;

// TODO: Use Table Associations
// TODO: Rename Tables and Remove Table Name

pub struct Edge<T> {
    pub cursor: String,
    pub node: T,
}

graphql_object!(Edge<Album>: GraphQLContext as "AlbumEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Album { &self.node }
});

graphql_object!(Edge<Artist>: GraphQLContext as "ArtistEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Artist { &self.node }
});

graphql_object!(Edge<Song>: GraphQLContext as "SongEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Song { &self.node }
});

graphql_object!(Edge<Playlist>: GraphQLContext as "PlaylistEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &Playlist { &self.node }
});

graphql_object!(Edge<PlaylistItem>: GraphQLContext as "PlaylistItemEdge" |&self| {
    field cursor() -> &str { &self.cursor }
    field node() -> &PlaylistItem { &self.node }
});

pub struct Connection<T> {
    pub count: usize,
    pub edges: Vec<Edge<T>>,
    pub has_next_page: bool,
}

graphql_object!(Connection<Album>: GraphQLContext as "AlbumConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Album>] { &self.edges }
    field page_info() -> PageInfo { PageInfo { has_next_page: self.has_next_page } }
});

graphql_object!(Connection<Artist>: GraphQLContext as "ArtistConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Artist>] { &self.edges }
    field page_info() -> PageInfo { PageInfo { has_next_page: self.has_next_page } }
});

graphql_object!(Connection<Song>: GraphQLContext as "SongConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Song>] { &self.edges }
    field page_info() -> PageInfo { PageInfo { has_next_page: self.has_next_page } }
});

graphql_object!(Connection<Playlist>: GraphQLContext as "PlaylistConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Playlist>] { &self.edges }
    field page_info() -> PageInfo { PageInfo { has_next_page: self.has_next_page } }
});

graphql_object!(Connection<PlaylistItem>: GraphQLContext as "PlaylistItemConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<PlaylistItem>] { &self.edges }
    field page_info() -> PageInfo { PageInfo { has_next_page: self.has_next_page } }
});

#[derive(GraphQLObject)]
pub struct PageInfo {
    pub has_next_page: bool,
}

#[derive(GraphQLInputObject)]
pub struct SortParams {
    pub sort_by: SortBy,

    #[graphql(default = "false")]
    pub reverse: bool,

    pub filter: Option<String>,
}

impl Default for SortParams {
    fn default() -> Self {
        SortParams {
            sort_by: SortBy::Lexicographically,
            reverse: false,
            filter: None,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum SortBy {
    #[graphql(name = "RECENTLY_ADDED")]
    RecentlyAdded,
    #[graphql(name = "LEXICOGRAPHICALLY")]
    Lexicographically,
    #[graphql(name = "RECENTLY_PLAYED")]
    RecentlyPlayed,
}

pub trait GetConnection<Table, ST, DB: Backend>: diesel::Queryable<ST, DB> + Sized {
    fn table() -> BoxedSelectStatement<'static, ST, Table, DB>;

    fn name() -> Box<Expression<SqlType = Text>>;

    fn time_added() -> Box<Expression<SqlType = Integer>>;

    fn last_played() -> Box<Expression<SqlType = Nullable<Integer>>>;

    fn get_connection(
        context: &GraphQLContext,
        first: i64,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Self>> {
        let conn = context.connection();
        let sort = sort.unwrap_or_default();

        let results_select_statement = GetConnection::table();
        let count_select_statement = GetConnection::table();

        let name = album::name; //GetConnection::name();
        let time_added = album::time_added; //GetConnection::time_added();
        let last_played = album::last_played; // GetConnection::last_played();

        let results_filtered =
            results_select_statement.filter(name.like(sort.filter.unwrap_or("%".to_string())));

        let count_filtered =
            count_select_statement.filter(name.like(sort.filter.unwrap_or("%".to_string())));

        let lower_bound: i64 = after.map_or(Ok(0), |offset| offset.parse())?;

        let bounded = results_filtered.limit(first).offset(lower_bound);

        let results: Vec<Self> = match sort.sort_by {
            SortBy::Lexicographically => {
                if !sort.reverse {
                    bounded.order_by(name.asc()).load(conn)
                } else {
                    bounded.order_by(name.desc()).load(conn)
                }
            }

            SortBy::RecentlyAdded => {
                if !sort.reverse {
                    bounded.order_by(time_added.desc()).load(conn)
                } else {
                    bounded.order_by(time_added.asc()).load(conn)
                }
            }

            SortBy::RecentlyPlayed => {
                if !sort.reverse {
                    bounded.order_by(last_played.desc()).load(conn)
                } else {
                    bounded.order_by(last_played.asc()).load(conn)
                }
            }
        }?;

        let count: i64 = count_filtered
            .select(dsl::count_star())
            .first(context.connection())?;

        // The exclusive upper bound of the window into the data.
        let upper_bound = lower_bound + first;

        let edges: Vec<Edge<Self>> = results
            .into_iter()
            .enumerate()
            .map(|(idx, node)| Edge {
                cursor: (lower_bound + idx as i64 + 1).to_string(),
                node,
            })
            .collect();

        Ok(Connection {
            count: count as usize,
            edges,
            has_next_page: upper_bound < count,
        })
    }
}
