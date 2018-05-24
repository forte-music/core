use forte_core::models::*;
use source_models::*;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongUserStatsSource {
    pub last_played: Option<i64>,
    pub play_count: Option<i32>,
    pub liked: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongSource {
    pub id: u64,
    pub name: String,
    pub duration: i32,
    pub track_number: Option<i32>,
    pub time_added: Option<i64>,

    pub artist_ids: Option<Vec<u64>>,
    pub album_id: u64,
    pub stats: Option<SongUserStatsSource>,
}

impl Into<Song> for SongSource {
    fn into(self) -> Song {
        Song {
            id: self.id.into(),
            name: self.name,
            album_id: self.album_id.into(),
            track_number: self.track_number.unwrap_or(1),
            duration: self.duration,
            time_added: self.time_added.unwrap_or(0).into_time(),
            play_count: self.stats
                .as_ref()
                .and_then(|stats| stats.play_count)
                .unwrap_or(0),
            last_played: self.stats
                .as_ref()
                .and_then(|stats| stats.last_played)
                .map(|t| t.into_time()),
            liked: self.stats
                .as_ref()
                .and_then(|stats| stats.liked)
                .unwrap_or(false),
            path: Path::new("").into(),
        }
    }
}
