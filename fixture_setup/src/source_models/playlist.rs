use forte_core::models::*;
use source_models::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSource {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub time_added: Option<i64>,
    pub song_ids: Vec<u64>,
    pub stats: Option<UserStatsSource>,
}

impl Into<Playlist> for PlaylistSource {
    fn into(self) -> Playlist {
        Playlist {
            id: self.id.into(),
            name: self.name,
            description: self.description.unwrap_or("".to_string()),
            time_added: self.time_added.unwrap_or(0).into_time(),
            last_played: self.stats
                .and_then(|stats| stats.last_played)
                .map(|t| t.into_time()),
        }
    }
}
