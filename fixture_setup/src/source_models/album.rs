use forte_core::models::*;
use source_models::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSource {
    pub id: u64,
    pub name: String,
    pub artwork_url: Option<String>,
    pub artist_id: u64,
    pub song_ids: Vec<u64>,
    pub release_year: Option<i32>,
    pub time_added: Option<i64>,
    pub stats: Option<UserStatsSource>,
}

impl Into<Album> for AlbumSource {
    fn into(self) -> Album {
        Album {
            id: self.id.into(),
            artwork_path: None,
            name: self.name,
            artist_id: self.artist_id.into(),
            release_year: self.release_year,
            time_added: self.time_added.unwrap_or(0).into_time(),
            last_played: self
                .stats
                .and_then(|stats| stats.last_played)
                .map(|t| t.into_time()),
        }
    }
}
