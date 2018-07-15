use serde::*;
use serde_json::*;

/// A beets database is made up of a list of these.
#[derive(Deserialize)]
pub struct Item {
    #[serde(rename = "album")]
    album_name: String,

    album_id: i32,

    #[serde(rename = "albumartist")]
    album_artist_name: String,

    #[serde(rename = "artist")]
    artist_name: String,

    year: i32,

    #[serde(rename = "title")]
    song_name: String,

    track: String,

    /// The duration of the track in hh:mm:ss
    length: String,
}
