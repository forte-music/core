CREATE TABLE album (
  id BINARY(128) PRIMARY KEY NOT NULL,
  artwork_path TEXT,
  name TEXT NOT NULL,
  artist_id BINARY(128) NOT NULL REFERENCES artist(id),
  release_year INTEGER,
  time_added TIMESTAMP NOT NULL,

  last_played TIMESTAMP,

  UNIQUE (name, artist_id)
);

CREATE TABLE artist (
  id BINARY(128) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  time_added TIMESTAMP NOT NULL,

  last_played TIMESTAMP,

  UNIQUE (name)
);

CREATE TABLE song_artist (
  song_id BINARY(128) NOT NULL REFERENCES song(id),
  artist_id BINARY(128) NOT NULL REFERENCES artist(id),
  PRIMARY KEY (song_id, artist_id)
);

CREATE TABLE song (
  id BINARY(128) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  album_id BINARY(128) NOT NULL REFERENCES album(id),
  track_number INTEGER NOT NULL,
  disk_number INTEGER NOT NULL,
  duration INTEGER NOT NULL,
  time_added TIMESTAMP NOT NULL,
  play_count INTEGER NOT NULL,
  last_played TIMESTAMP,
  liked BOOLEAN NOT NULL,
  path TEXT UNIQUE NOT NULL,

  UNIQUE(track_number, disk_number, album_id)
);

CREATE TABLE playlist (
  id BINARY(128) PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  time_added TIMESTAMP NOT NULL,

  last_played TIMESTAMP
);

CREATE TABLE playlist_item (
  id BINARY(128) PRIMARY KEY NOT NULL,
  playlist_id BINARY(128) NOT NULL REFERENCES playlist(id),
  rank TEXT NOT NULL,
  song_id BINARY(128) NOT NULL REFERENCES song(id)
);
