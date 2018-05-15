pub mod album;
pub mod artist;
pub mod connection;
pub mod mutation;
pub mod playlist;
pub mod query;
pub mod song;
pub mod stats;

pub use self::album::*;
pub use self::artist::*;
pub use self::connection::*;
pub use self::mutation::*;
pub use self::playlist::*;
pub use self::query::*;
pub use self::song::*;
pub use self::stats::*;

use juniper::FieldError;
use juniper::FieldResult;

pub fn NotImplementedErr<T>() -> FieldResult<T> {
    Err(FieldError::from("Not Implemented"))
}
