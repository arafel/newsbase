-- Your SQL goes here
ALTER TABLE newsgroups
ADD COLUMN last_high INTEGER
DEFAULT 0
NOT NULL