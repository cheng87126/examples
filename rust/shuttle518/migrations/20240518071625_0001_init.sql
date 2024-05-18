-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id serial PRIMARY KEY,
  pwd TEXT NOT NULL,
  user_name TEXT NOT NULL
);