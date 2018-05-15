use context::GraphQLContext;
use models::*;

pub struct Edge<T> {
    pub cursor: String,
    pub node: T,
}

pub struct Connection<T> {
    pub count: usize,
    pub edges: Vec<Edge<T>>,
}

#[derive(GraphQLInputObject)]
pub struct ConnectionQuery {
    #[graphql(default = "25")]
    pub limit: i32,

    #[graphql(default = "None")]
    pub cursor: Option<String>,
}

#[derive(GraphQLInputObject)]
pub struct SortParams {
    pub sort_by: SortBy,

    #[graphql(default = "false")]
    pub reverse: bool,

    #[graphql(default = "String::new()")]
    pub filter: String,
}

#[derive(GraphQLEnum)]
pub enum SortBy {
    #[graphql(name = "RECENTLY_ADDED")]
    RecentlyAdded,
    #[graphql(name = "LEXICOGRAPHICALLY")]
    Lexicographically,
    #[graphql(name = "RELEVANCE")]
    Relevance,
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

graphql_object!(Connection<Album>: GraphQLContext as "AlbumConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Album>] { &self.edges }
});

graphql_object!(Connection<Artist>: GraphQLContext as "ArtistConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Artist>] { &self.edges }
});

graphql_object!(Connection<Song>: GraphQLContext as "SongConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Song>] { &self.edges }
});

graphql_object!(Connection<Playlist>: GraphQLContext as "PlaylistConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<Playlist>] { &self.edges }
});

graphql_object!(Connection<PlaylistItem>: GraphQLContext as "PlaylistItemConnection" |&self| {
    field count() -> i32 { self.count as i32 }
    field edges() -> &[Edge<PlaylistItem>] { &self.edges }
});
