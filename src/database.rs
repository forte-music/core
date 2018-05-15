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
        time_added -> Integer,
    }
}

table! {
    playlist_item (id) {
        id -> Text,
        rank -> Text,
        song_id -> Text,
    }
}

table! {
    song (id) {
        id -> Text,
        name -> Text,
        album_id -> Text,
        stat_id -> Nullable<Text>,
        track_number -> Integer,
        disk_number -> Integer,
        duration -> Integer,
        time_added -> Integer,
    }
}

table! {
    song_user_stats (id) {
        id -> Text,
        play_count -> Integer,
        last_played -> Nullable<Integer>,
        liked -> Bool,
    }
}

joinable!(album -> artist (artist_id));
joinable!(playlist_item -> song (song_id));
joinable!(song -> album (album_id));
joinable!(song -> song_user_stats (stat_id));

allow_tables_to_appear_in_same_query!(
    album,
    artist,
    playlist,
    playlist_item,
    song,
    song_user_stats,
);
