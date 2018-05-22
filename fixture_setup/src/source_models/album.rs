use forte_core::models::*;
use source_models::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSource {
    pub id: String,
    pub name: String,
    pub artwork_url: Option<String>,
    pub artist_id: String,
    pub song_ids: Vec<String>,
    pub release_year: i32,
    pub time_added: Option<i32>,
    pub stats: Option<UserStatsSource>,
}

impl Into<Album> for AlbumSource {
    fn into(self) -> Album {
        Album {
            id: self.id,
            artwork_url: self.artwork_url,
            name: self.name,
            artist_id: self.artist_id,
            release_year: self.release_year,
            time_added: self.time_added.unwrap_or(0),
            last_played: self.stats.and_then(|stats| stats.last_played),
        }
    }
}
