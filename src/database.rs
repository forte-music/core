table! {
    album (id) {
        id -> Text,
        artwork_url -> Nullable<Text>,
        name -> Text,
        artist_id -> Text,
        release_year -> Integer,
        time_added -> Integer,
    }
}

table! {
    artist (id) {
        id -> Text,
        name -> Text,
        time_added -> Integer,
    }
}

table! {
    playlist (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        time_added -> Integer,
    }
}

table! {
    playlist_item (id) {
        id -> Text,
        playlist_id -> Text,
        rank -> Text,
        song_id -> Text,
    }
}

table! {
    song (id) {
        id -> Text,
        name -> Text,
        album_id -> Text,
        track_number -> Integer,
        disk_number -> Integer,
        duration -> Integer,
        time_added -> Integer,
        play_count -> Integer,
        last_played -> Nullable<Integer>,
        liked -> Bool,
    }
}

table! {
    song_artist (song_id, artist_id) {
        song_id -> Text,
        artist_id -> Text,
    }
}

joinable!(album -> artist (artist_id));
joinable!(playlist_item -> playlist (playlist_id));
joinable!(playlist_item -> song (song_id));
joinable!(song -> album (album_id));
joinable!(song_artist -> artist (artist_id));
joinable!(song_artist -> song (song_id));

allow_tables_to_appear_in_same_query!(
    album,
    artist,
    playlist,
    playlist_item,
    song,
    song_artist,
);
