table! {
    album (id) {
        id -> Binary,
        artwork_url -> Nullable<Text>,
        name -> Text,
        artist_id -> Binary,
        release_year -> Integer,
        time_added -> Timestamp,
        last_played -> Nullable<Timestamp>,
    }
}

table! {
    artist (id) {
        id -> Binary,
        name -> Text,
        time_added -> Timestamp,
        last_played -> Nullable<Timestamp>,
    }
}

table! {
    playlist (id) {
        id -> Binary,
        name -> Text,
        description -> Text,
        time_added -> Timestamp,
        last_played -> Nullable<Timestamp>,
    }
}

table! {
    playlist_item (id) {
        id -> Binary,
        playlist_id -> Binary,
        rank -> Text,
        song_id -> Binary,
    }
}

table! {
    song (id) {
        id -> Binary,
        name -> Text,
        album_id -> Binary,
        track_number -> Integer,
        disk_number -> Integer,
        duration -> Integer,
        time_added -> Timestamp,
        play_count -> Integer,
        last_played -> Nullable<Timestamp>,
        liked -> Bool,
    }
}

table! {
    song_artist (song_id, artist_id) {
        song_id -> Binary,
        artist_id -> Binary,
    }
}

joinable!(album -> artist (artist_id));
joinable!(playlist_item -> playlist (playlist_id));
joinable!(playlist_item -> song (song_id));
joinable!(song -> album (album_id));
joinable!(song_artist -> artist (artist_id));
joinable!(song_artist -> song (song_id));

allow_tables_to_appear_in_same_query!(album, artist, playlist, playlist_item, song, song_artist,);
