use source_models::*;

#[derive(Deserialize)]
pub struct Import {
    pub albums: Option<Vec<AlbumSource>>,
    pub artists: Option<Vec<ArtistSource>>,
    pub playlists: Option<Vec<PlaylistSource>>,
    pub songs: Option<Vec<SongSource>>,
}
