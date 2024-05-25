-- Add migration script here
CREATE TABLE IF NOT EXISTS urls (
  id serial PRIMARY KEY,
  content TEXT NOT NULL,
  remark TEXT NOT NULL,
  user_id INTEGER NOT NULL
);