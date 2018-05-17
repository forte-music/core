CREATE TABLE album (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  artwork_url TEXT,
  name TEXT NOT NULL,
  artist_id VARCHAR(36) NOT NULL REFERENCES artist(id),
  release_year INTEGER NOT NULL,
  time_added INTEGER NOT NULL,

  last_played INTEGER
);

CREATE TABLE artist (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  time_added INTEGER NOT NULL,

  last_played INTEGER
);

CREATE TABLE song_artist (
  song_id VARCHAR(36) NOT NULL REFERENCES song(id),
  artist_id VARCHAR(36) NOT NULL REFERENCES artist(id),
  PRIMARY KEY (song_id, artist_id)
);

CREATE TABLE song (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  album_id VARCHAR(36) NOT NULL REFERENCES album(id),
  track_number INTEGER NOT NULL,
  disk_number INTEGER NOT NULL,
  duration INTEGER NOT NULL,
  time_added INTEGER NOT NULL,
  play_count INTEGER NOT NULL,
  last_played INTEGER,
  liked BOOLEAN NOT NULL
);

CREATE TABLE playlist (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  time_added INTEGER NOT NULL,

  last_played INTEGER
);

CREATE TABLE playlist_item (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  playlist_id VARCHAR(36) NOT NULL REFERENCES playlist(id),
  rank TEXT NOT NULL,
  song_id VARCHAR(36) NOT NULL REFERENCES song(id)
);
