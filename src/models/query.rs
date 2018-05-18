use context::GraphQLContext;
use juniper::{FieldResult, ID};
use models::*;

use database::album;
use diesel::dsl;
use diesel::prelude::*;

use models::Connection;

pub struct Query;

impl Query {
    pub fn album(context: &GraphQLContext, id: &str) -> FieldResult<Album> {
        Album::from_id(context, id)
    }

    pub fn albums(
        context: &GraphQLContext,
        first: i64,
        after: Option<String>,
        sort: Option<SortParams>,
    ) -> FieldResult<Connection<Album>> {
        let conn = context.connection();

        let sort = sort.unwrap_or(SortParams {
            sort_by: SortBy::Lexicographically,
            reverse: false,
            filter: None,
        });

        let filtered =
            album::table.filter(album::name.like(sort.filter.unwrap_or("%".to_string())));

        let lower_bound: i64 = if let Some(offset) = after {
            offset.parse()?
        } else {
            0
        };

        let bounded = filtered.clone().limit(first).offset(lower_bound);

        let results: Vec<Album> = match sort.sort_by {
            SortBy::Lexicographically => {
                if !sort.reverse {
                    bounded.order_by(album::name.asc()).load(conn)
                } else {
                    bounded.order_by(album::name.desc()).load(conn)
                }
            }

            SortBy::RecentlyAdded => {
                if !sort.reverse {
                    bounded.order_by(album::time_added.desc()).load(conn)
                } else {
                    bounded.order_by(album::time_added.asc()).load(conn)
                }
            }

            SortBy::RecentlyPlayed => {
                if !sort.reverse {
                    bounded.order_by(album::last_played.desc()).load(conn)
                } else {
                    bounded.order_by(album::last_played.asc()).load(conn)
                }
            }
        }?;

        let count: i64 = filtered
            .select(dsl::count_star())
            .first(context.connection())?;

        // The exclusive upper bound of the window into the data.
        let upper_bound = lower_bound + first;

        let edges: Vec<Edge<Album>> = results
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

    pub fn artist(context: &GraphQLContext, id: &str) -> FieldResult<Artist> {
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

    pub fn song(context: &GraphQLContext, id: &str) -> FieldResult<Song> {
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

    pub fn playlist(context: &GraphQLContext, id: &str) -> FieldResult<Playlist> {
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
    field album(&executor, id: ID) -> FieldResult<Album> {
        Query::album(executor.context(), &id)
    }

    field albums(
        &executor,
        first = 25: i32,
        after: Option<String>,
        sort: Option<SortParams>
    ) -> FieldResult<Connection<Album>> {
        Query::albums(executor.context(), first as i64, after, sort)
    }

    field artist(&executor, id: ID) -> FieldResult<Artist> {
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

    field song(&executor, id: ID) -> FieldResult<Song> {
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

    field playlist(&executor, id: ID) -> FieldResult<Playlist> {
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
