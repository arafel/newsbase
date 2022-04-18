-- Your SQL goes here
CREATE TABLE newsgroups (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  low INTEGER NOT NULL,
  high INTEGER NOT NULL
)