use context::GraphQLContext;
use juniper::FieldResult;
use models::*;

use diesel::associations::HasTable;
use diesel::dsl;
use diesel::prelude::*;
use diesel::query_builder::AsQuery;

use database::album;

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

trait GetConnection<T>
where
    Self: Sized + HasTable<Table = T>,
    T: Table + AsQuery + QueryDsl,
{
    fn get_connection(
        context: &GraphQLContext,
        first: i64,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Self>> {
        let conn = context.connection();
        let sort = sort.unwrap_or_default();
        let lower_bound: i64 = after.map_or(Ok(0), |offset| offset.parse())?;

        let name = album::name;
        let time_added = album::time_added;
        let last_played = album::last_played;

        let table: T = <Self as HasTable>::table();
        let mut query = table.into_boxed();
        if let Some(ref filter) = sort.filter {
            query = query.filter(name.like(filter));
        }

        query = match sort.sort_by {
            SortBy::Lexicographically => {
                if !sort.reverse {
                    query.order_by(name.asc())
                } else {
                    query.order_by(name.desc())
                }
            }

            SortBy::RecentlyAdded => {
                if !sort.reverse {
                    query.order_by(time_added.desc())
                } else {
                    query.order_by(time_added.asc())
                }
            }

            SortBy::RecentlyPlayed => {
                if !sort.reverse {
                    query.order_by(last_played.desc())
                } else {
                    query.order_by(last_played.asc())
                }
            }
        };

        let results: Vec<Self> = query.limit(first).offset(lower_bound).load(conn)?;

        let mut count_query = HasTable::table().into_boxed();
        if let Some(ref filter) = sort.filter {
            count_query = count_query.filter(name.like(filter));
        }

        let count: i64 = count_query
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

#[derive(GraphQLEnum)]
pub enum SortBy {
    #[graphql(name = "RECENTLY_ADDED")]
    RecentlyAdded,
    #[graphql(name = "LEXICOGRAPHICALLY")]
    Lexicographically,
    #[graphql(name = "RECENTLY_PLAYED")]
    RecentlyPlayed,
}
