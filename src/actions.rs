use schema::model::*;
use redis::{Connection, Commands, RedisResult};

fn add_list_to_set<T: Keyed>(set_key: &str, list: &[String], db: &Connection) -> RedisResult<()> {
    if list.len() == 0 {
        return Ok(());
    }

    db.sadd::<_, _, ()>(set_key, list)?;

    Ok(())
}

pub fn add_album(album: &Album, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(Album::key(&album.id), &[
        ("id", &album.id),
        ("artwork_url", &album.artwork_url.clone().unwrap_or(String::new())),
        ("name", &album.name),
        ("artist_id", &album.artist_id),
        ("release_year", &album.release_year.to_string()),
        ("time_added", &album.time_added.to_string())
    ])?;

    db.sadd::<_, _, ()>("albums", &album.id)?;

    Ok(())
}

pub fn add_songs_to_album(album_id: &str, songs_ids: &[String], db: &Connection) -> RedisResult<()> {
    add_list_to_set::<Album>(&Album::songs_key(album_id), songs_ids, db)
}

pub fn add_artist(artist: &Artist, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(Artist::key(&artist.id), &[
        ("id", &artist.id),
        ("name", &artist.name),
        ("time_added", &artist.time_added.to_string())
    ])?;

    db.sadd::<_, _, ()>("artists", &artist.id)?;

    Ok(())
}

pub fn add_albums_to_artist(artist_id: &str, album_ids: &[String], db: &Connection) -> RedisResult<()> {
    add_list_to_set::<Artist>(&Artist::albums_key(artist_id), album_ids, db)
}

pub fn add_song(song: &Song, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(Song::key(&song.id), &[
        ("id", &song.id),
        ("name", &song.name),
        ("album_id", &song.album_id),
        ("stat_id", &song.stat_id),
        ("stream_url", &song.stream_url),
        ("track_number", &song.track_number.to_string()),
        ("disk_number", &song.disk_number.to_string()),
        ("duration", &song.duration.to_string()),
        ("time_added", &song.time_added.to_string())
    ])?;

    db.sadd::<_, _, ()>("songs", &song.id)?;

    Ok(())
}

pub fn add_artists_to_song(song_id: &str, artist_ids: &[String], db: &Connection) -> RedisResult<()> {
    add_list_to_set::<Song>(&Song::artists_key(song_id), artist_ids, db)
}

pub fn add_song_stats(song_stats: &SongUserStats, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(SongUserStats::key(&song_stats.id), &[
        ("id", &song_stats.id),
        ("play_count", &song_stats.play_count.to_string()),
        ("liked", &song_stats.liked.to_string()),
    ])?;

    db.hset::<_, _, _, ()>(SongUserStats::key(&song_stats.id), "last_played", song_stats.last_played)?;

    Ok(())
}

pub fn add_playlist(playlist: &Playlist, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(Playlist::key(&playlist.id), &[
        ("id", &playlist.id),
        ("name", &playlist.name),
        ("time_added", &playlist.time_added.to_string())
    ])?;

    db.sadd::<_, _, ()>("playlists", &playlist.id)?;

    Ok(())
}

pub fn add_playlist_items_to_playlist(playlist_id: &str, playlist_item_ids: &[String], db: &Connection)
        -> RedisResult<()> {
    if playlist_item_ids.len() == 0 {
        return Ok(());
    }

    db.lpush(Playlist::key(playlist_id), playlist_item_ids)
}

pub fn add_playlist_item(playlist_item: &PlaylistItem, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(PlaylistItem::key(&playlist_item.id), &[
        ("id", &playlist_item.id),
        ("song_id", &playlist_item.song_id)
    ])?;

    db.sadd::<_, _, ()>("playlist-items", &playlist_item.id)?;

    Ok(())
}
