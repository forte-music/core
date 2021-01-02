pub mod album;
pub mod artist;
pub mod connection;
pub mod id;
pub mod mutation;
pub mod path;
pub mod query;
pub mod recents;
pub mod song;
pub mod song_user_stats;
pub mod stats_collection;
pub mod user_stats;

pub use self::album::*;
pub use self::artist::*;
pub use self::connection::*;
pub use self::id::*;
pub use self::mutation::*;
pub use self::path::*;
pub use self::query::*;
pub use self::recents::*;
pub use self::song::*;
pub use self::song_user_stats::*;
pub use self::stats_collection::*;
pub use self::user_stats::*;
pub use chrono::NaiveDateTime;

use crate::context::GraphQLContext;
use juniper::{EmptySubscription, RootNode};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
