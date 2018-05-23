use forte_core::models::*;
use source_models::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSource {
    pub id: u64,
    pub name: String,
    pub time_added: Option<i64>,
    pub album_ids: Vec<u64>,
    pub stats: Option<UserStatsSource>,
}

impl Into<Artist> for ArtistSource {
    fn into(self) -> Artist {
        Artist {
            id: self.id.into(),
            name: self.name,
            time_added: self.time_added.unwrap_or(0).into_time(),
            last_played: self.stats
                .and_then(|stats| stats.last_played)
                .map(|t| t.into_time()),
        }
    }
}
