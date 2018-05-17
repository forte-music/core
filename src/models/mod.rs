pub mod album;
pub mod artist;
pub mod connection;
pub mod mutation;
pub mod playlist;
pub mod query;
pub mod song;
pub mod user_stats;
pub mod song_user_stats;
pub mod stats_collection;

pub use self::album::*;
pub use self::artist::*;
pub use self::connection::*;
pub use self::mutation::*;
pub use self::playlist::*;
pub use self::query::*;
pub use self::song::*;
pub use self::user_stats::*;
pub use self::song_user_stats::*;
pub use self::stats_collection::*;

use juniper::FieldError;
use juniper::FieldResult;

pub fn NotImplementedErr<T>() -> FieldResult<T> {
    Err(FieldError::from("Not Implemented"))
}
