-- Your SQL goes here
CREATE TABLE tags (
  id INTEGER PRIMARY KEY,
  label TEXT UNIQUE NOT NULL,
  color TEXT
);

CREATE TABLE folders (
  id INTEGER PRIMARY KEY,
  label VARCHAR(256),
  parent INTEGER,
  FOREIGN KEY(parent) REFERENCES folders(id)
);

CREATE TABLE bookmarks (
  id INTEGER PRIMARY KEY,
  url TEXT NOT NULL,
  label VARCHAR(256) UNIQUE,
  folder INTEGER,
  FOREIGN KEY (folder) REFERENCES folder(id)
);

CREATE TABLE bookmark_tag_map (
  id INTEGER PRIMARY KEY,
  bookmark_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL
);
