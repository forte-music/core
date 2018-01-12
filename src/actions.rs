use schema::model::*;
use redis::{Connection, Commands, RedisResult};

fn add_list_to_set<T: Keyed>(set_key: &str, list: &[String], db: &Connection) -> RedisResult<()> {
    db.sadd::<_, _, ()>(set_key, list)?;

    Ok(())
}

pub fn add_album(album: &Album, db: &Connection) -> RedisResult<()> {
    db.hset_multiple::<_, _, _, ()>(Album::key(&album.id), &[
        ("id", &album.id),
        ("artwork_url", &album.artwork_url.clone().unwrap_or(String::new())),
        ("name", &album.name),
        ("artist_id", &album.artist_id),
        ("release_year", &album.release_year.to_string())
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
        ("name", &artist.name)
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
        ("duration", &song.duration.to_string())
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
        ("liked", &song_stats.liked.to_string())
    ])?;

    db.hset::<_, _, _, ()>(SongUserStats::key(&song_stats.id), "last_played", song_stats.last_played)?;

    Ok(())
}
