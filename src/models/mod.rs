pub mod album;
pub mod artist;
pub mod connection;
pub mod id;
pub mod mutation;
pub mod path;
pub mod playlist;
pub mod query;
pub mod song;
pub mod song_user_stats;
pub mod stats_collection;
pub mod time;
pub mod user_stats;

pub use self::album::*;
pub use self::artist::*;
pub use self::connection::*;
pub use self::id::*;
pub use self::mutation::*;
pub use self::path::*;
pub use self::playlist::*;
pub use self::query::*;
pub use self::song::*;
pub use self::song_user_stats::*;
pub use self::stats_collection::*;
pub use self::time::*;
pub use self::user_stats::*;
pub use chrono::NaiveDateTime;

use juniper::FieldError;
use juniper::FieldResult;
use juniper::RootNode;

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

pub fn NotImplementedErr<T>() -> FieldResult<T> {
    Err(FieldError::from("Not Implemented"))
}
