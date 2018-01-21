use forte_core::schema::model::*;

extern crate serde;

#[derive(Deserialize)]
pub struct ArtistImport {
    pub artists: Vec<ArtistSource>,
}

#[derive(Deserialize)]
pub struct AlbumImport {
    pub albums: Vec<AlbumSource>,
}

#[derive(Deserialize)]
pub struct PlaylistImport {
    pub playlists: Vec<PlaylistSource>,
}

#[derive(Deserialize)]
pub struct SongImport {
    pub songs: Vec<SongSource>,
}

#[derive(Deserialize)]
pub struct StatsImport {
    pub stats: Vec<StatsSource>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSource {
    pub id: String,
    pub name: String,
    pub album_ids: Vec<String>,
    pub time_added: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSource {
    pub id: String,
    pub name: String,
    pub artwork_url: String,
    pub artist_id: String,
    pub song_ids: Vec<String>,
    pub release_year: i32,
    pub time_added: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSource {
    pub id: String,
    pub name: String,
    pub song_ids: Vec<String>,
    pub time_added: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongSource {
    pub id: String,
    pub stream_url: String,
    pub name: String,
    pub duration: i32,
    pub track_number: Option<i32>,
    pub disk_number: Option<i32>,
    pub artist_ids: Option<Vec<String>>,
    pub album_id: String,
    pub stats_id: String,
    pub time_added: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsSource {
    pub id: String,
    pub play_count: i32,
    pub last_played: Option<i32>,
    pub liked: bool,
}

impl Into<Artist> for ArtistSource {
    fn into(self) -> Artist {
        Artist {
            id: self.id,
            name: self.name,
            time_added: self.time_added.unwrap_or(0),
        }
    }
}

impl Into<Album> for AlbumSource {
    fn into(self) -> Album {
        Album {
            id: self.id,
            name: self.name,
            artwork_url: Some(self.artwork_url),
            artist_id: self.artist_id,
            release_year: self.release_year,
            time_added: self.time_added.unwrap_or(0),
        }
    }
}

impl Into<Playlist> for PlaylistSource {
    fn into(self) -> Playlist {
        Playlist {
            id: self.id,
            name: self.name,
            time_added: self.time_added.unwrap_or(0),
        }
    }
}

impl Into<Song> for SongSource {
    fn into(self) -> Song {
        Song {
            id: self.id,
            name: self.name,
            album_id: self.album_id,
            stat_id: self.stats_id,
            stream_url: self.stream_url,
            track_number: self.track_number.unwrap_or(1),
            disk_number: self.disk_number.unwrap_or(1),
            duration: self.duration,
            time_added: self.time_added.unwrap_or(0),
        }
    }
}

impl Into<SongUserStats> for StatsSource {
    fn into(self) -> SongUserStats {
        SongUserStats {
            id: self.id,
            play_count: self.play_count,
            last_played: self.last_played,
            liked: self.liked,
        }
    }
}
