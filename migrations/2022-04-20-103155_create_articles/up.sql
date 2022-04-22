-- Your SQL goes here
CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  newsgroup_id INTEGER REFERENCES newsgroups(id) NOT NULL,
  server_id INTEGER NOT NULL,
  author TEXT NOT NULL,
  subject TEXT NOT NULL,
  date_sent TEXT NOT NULL
)