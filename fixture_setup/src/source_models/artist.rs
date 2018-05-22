use forte_core::models::*;
use source_models::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSource {
    pub id: String,
    pub name: String,
    pub time_added: Option<i32>,
    pub album_ids: Vec<String>,
    pub stats: Option<UserStatsSource>,
}

impl Into<Artist> for ArtistSource {
    fn into(self) -> Artist {
        Artist {
            id: self.id,
            name: self.name,
            time_added: self.time_added.unwrap_or(0),
            last_played: self.stats.and_then(|stats| stats.last_played),
        }
    }
}
