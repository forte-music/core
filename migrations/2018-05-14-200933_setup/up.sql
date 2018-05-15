CREATE TABLE album (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  artwork_url TEXT,
  name TEXT NOT NULL,
  artist_id VARCHAR(36) NOT NULL REFERENCES artist(id),
  release_year INTEGER NOT NULL,
  time_added INTEGER NOT NULL
);

CREATE TABLE artist (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  time_added INTEGER NOT NULL
);

CREATE TABLE song (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  album_id VARCHAR(36) NOT NULL REFERENCES album(id),
  stat_id VARCHAR(36) REFERENCES song_user_stats(id),
  track_number INTEGER NOT NULL,
  disk_number INTEGER NOT NULL,
  duration INTEGER NOT NULL,
  time_added INTEGER NOT NULL
);

CREATE TABLE song_user_stats (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  play_count INTEGER NOT NULL,
  last_played INTEGER,
  liked BOOLEAN NOT NULL
);

CREATE TABLE playlist (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  time_added INTEGER NOT NULL
);

CREATE TABLE playlist_item (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  rank TEXT NOT NULL,
  song_id VARCHAR(36) NOT NULL REFERENCES song(id)
);
