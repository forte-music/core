use forte_core::models::*;
use source_models::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSource {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub time_added: Option<i32>,
    pub song_ids: Vec<String>,
    pub stats: Option<UserStatsSource>,
}

impl Into<Playlist> for PlaylistSource {
    fn into(self) -> Playlist {
        Playlist {
            id: self.id,
            name: self.name,
            description: self.description.unwrap_or("".to_string()),
            time_added: self.time_added.unwrap_or(0),
            last_played: self.stats.and_then(|stats| stats.last_played),
        }
    }
}