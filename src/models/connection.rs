use crate::context::GraphQLContext;
use crate::models::*;
use diesel::associations::HasTable;
use diesel::dsl;
use diesel::expression::NonAggregate;
use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use diesel::query_builder::BoxedSelectStatement;
use diesel::query_builder::QueryFragment;
use diesel::query_dsl::boxed_dsl::BoxedDsl;
use diesel::sql_types::HasSqlType;
use diesel::sql_types::Nullable;
use diesel::sql_types::Text;
use diesel::sql_types::Timestamp;
use diesel::sqlite::Sqlite;
use juniper::{graphql_object, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject};

pub struct Edge<T> {
    pub cursor: String,
    pub node: T,
}

#[graphql_object(name = "AlbumEdge", context = GraphQLContext)]
impl Edge<Album> {
    fn cursor(&self) -> &str {
        &self.cursor
    }
    fn node(&self) -> &Album {
        &self.node
    }
}

#[graphql_object(name = "ArtistEdge", context = GraphQLContext)]
impl Edge<Artist> {
    fn cursor(&self) -> &str {
        &self.cursor
    }
    fn node(&self) -> &Artist {
        &self.node
    }
}

#[graphql_object(name = "SongEdge", context = GraphQLContext)]
impl Edge<Song> {
    fn cursor(&self) -> &str {
        &self.cursor
    }
    fn node(&self) -> &Song {
        &self.node
    }
}

pub struct Connection<T> {
    pub count: usize,
    pub edges: Vec<Edge<T>>,
    pub has_next_page: bool,
}

#[graphql_object(name = "AlbumConnection", context = GraphQLContext)]
impl Connection<Album> {
    fn count(&self) -> i32 {
        self.count as i32
    }
    fn edges(&self) -> &[Edge<Album>] {
        &self.edges
    }
    fn page_info(&self) -> PageInfo {
        PageInfo {
            has_next_page: self.has_next_page,
        }
    }
}

#[graphql_object(name = "ArtistConnection", context = GraphQLContext)]
impl Connection<Artist> {
    fn count(&self) -> i32 {
        self.count as i32
    }
    fn edges(&self) -> &[Edge<Artist>] {
        &self.edges
    }
    fn page_info(&self) -> PageInfo {
        PageInfo {
            has_next_page: self.has_next_page,
        }
    }
}

#[graphql_object(name = "SongConnection", context = GraphQLContext)]
impl Connection<Song> {
    fn count(&self) -> i32 {
        self.count as i32
    }
    fn edges(&self) -> &[Edge<Song>] {
        &self.edges
    }
    fn page_info(&self) -> PageInfo {
        PageInfo {
            has_next_page: self.has_next_page,
        }
    }
}

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

pub trait GetConnection<TB>
where
    Self: HasTable<Table = TB> + Queryable<TB::SqlType, Sqlite> + Sized,
    TB: Table
        + BoxedDsl<
            'static,
            Sqlite,
            Output = BoxedSelectStatement<'static, <TB as AsQuery>::SqlType, TB, Sqlite>,
        >,
    <TB as QuerySource>::FromClause: QueryFragment<Sqlite>,
    Sqlite: HasSqlType<TB::SqlType>,
{
    type Name: Column<Table = TB, SqlType = Text>
        + AppearsOnTable<TB>
        + QueryFragment<Sqlite>
        + NonAggregate;
    type TimeAdded: Column<Table = TB, SqlType = Timestamp>
        + AppearsOnTable<TB>
        + QueryFragment<Sqlite>;
    type LastPlayed: Column<Table = TB, SqlType = Nullable<Timestamp>>
        + AppearsOnTable<TB>
        + QueryFragment<Sqlite>;

    fn name() -> Self::Name;
    fn time_added() -> Self::TimeAdded;
    fn last_played() -> Self::LastPlayed;

    fn get_connection(
        context: &GraphQLContext,
        first: i64,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Self>> {
        let conn = &context.connection() as &SqliteConnection;
        let sort = sort.unwrap_or_default();
        let lower_bound = after.map_or(Ok(0), |offset| offset.parse())?;

        let mut query: BoxedSelectStatement<'_, <TB as AsQuery>::SqlType, TB, Sqlite> =
            Self::table().into_boxed();
        if let Some(ref filter) = sort.filter {
            query = QueryDsl::filter(query, Self::name().like(format!("%{}%", filter)));
        }

        query = match sort.sort_by {
            SortBy::Lexicographically => {
                if !sort.reverse {
                    query.order_by(Self::name().asc())
                } else {
                    query.order_by(Self::name().desc())
                }
            }

            SortBy::RecentlyAdded => {
                if !sort.reverse {
                    query.order_by(Self::time_added().desc())
                } else {
                    query.order_by(Self::time_added().asc())
                }
            }

            SortBy::RecentlyPlayed => {
                if !sort.reverse {
                    query.order_by(Self::last_played().desc())
                } else {
                    query.order_by(Self::last_played().asc())
                }
            }
        };

        let results: Vec<Self> = query.limit(first).offset(lower_bound).load(conn)?;

        let mut count_query = Self::table().into_boxed();
        if let Some(ref filter) = sort.filter {
            count_query = QueryDsl::filter(count_query, Self::name().like(filter));
        }

        let count: i64 = count_query.select(dsl::count_star()).first(conn)?;

        // The exclusive upper bound of the window into the data.
        let upper_bound = lower_bound + if first < 0 { count } else { first };

        let edges: Vec<Edge<Self>> = results
            .into_iter()
            .enumerate()
            .map(|(idx, album)| Edge {
                cursor: (lower_bound + idx as i64 + 1).to_string(),
                node: album,
            })
            .collect();

        Ok(Connection {
            count: count as usize,
            edges,
            has_next_page: upper_bound < count,
        })
    }
}
