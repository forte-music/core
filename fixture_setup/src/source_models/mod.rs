pub mod album;
pub mod artist;
pub mod import;
pub mod playlist;
pub mod song;
pub mod user_stats;

pub use self::album::*;
pub use self::artist::*;
pub use self::import::*;
pub use self::playlist::*;
pub use self::song::*;
pub use self::user_stats::*;

pub use serde::de::Deserialize;

use chrono::NaiveDateTime;

pub trait IntoTime {
    fn into_time(self) -> NaiveDateTime;
}

impl IntoTime for i64 {
    fn into_time(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self, 0)
    }
}
