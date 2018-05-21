use forte_core::models::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongUserStatsSource {
    pub last_played: Option<i32>,
    pub play_count: Option<i32>,
    pub liked: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongSource {
    pub id: String,
    pub name: String,
    pub duration: i32,
    pub track_number: Option<i32>,
    pub disk_number: Option<i32>,
    pub time_added: Option<i32>,

    pub artist_ids: Option<Vec<String>>,
    pub album_id: String,
    pub stats: Option<SongUserStatsSource>,
}

impl Into<Song> for SongSource {
    fn into(self) -> Song {
        Song {
            id: self.id,
            name: self.name,
            album_id: self.album_id,
            track_number: self.track_number.unwrap_or(1),
            disk_number: self.disk_number.unwrap_or(1),
            duration: self.duration,
            time_added: self.time_added.unwrap_or(0),
            play_count: self.stats
                .as_ref()
                .and_then(|stats| stats.play_count)
                .unwrap_or(0),
            last_played: self.stats.as_ref().and_then(|stats| stats.last_played),
            liked: self.stats
                .as_ref()
                .and_then(|stats| stats.liked)
                .unwrap_or(false),
        }
    }
}
